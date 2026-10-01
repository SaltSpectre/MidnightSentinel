//! Core overlay logic: shared by the standalone `midsentcli` binary and by
//! `midsent` (the tray controller, which runs it in-process on a background
//! thread — or synchronously under `--run-now` — to avoid the latency of
//! spawning a whole new process).
//!
//! Creates one borderless, topmost, pure-black window per monitor. Double-
//! clicking any overlay (any mouse button) — or calling [`dismiss_running_overlay`]
//! from elsewhere, which is what a second `run_overlay`/`midsentcli` launch
//! does — closes all of them and [`run_overlay`] returns.

use std::cell::RefCell;
use std::sync::OnceLock;

use windows::core::{w, BOOL, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetStockObject, BLACK_BRUSH, HBRUSH, HDC, HMONITOR,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, FindWindowW, GetMessageW,
    PostMessageW, PostQuitMessage, RegisterClassW, RegisterWindowMessageW, SetCursor,
    SetForegroundWindow, ShowCursor, ShowWindow, TranslateMessage, CS_DBLCLKS, MSG, SW_SHOW,
    WM_DESTROY, WM_LBUTTONDBLCLK, WM_MBUTTONDBLCLK, WM_RBUTTONDBLCLK, WM_SETCURSOR, WNDCLASSW,
    WS_EX_TOPMOST, WS_POPUP,
};

/// Name of the named OS mutex that guards "only one overlay display active
/// at a time". Shared by the standalone binary and by the tray controller's
/// in-process invocation so the two launch paths can't run simultaneously.
pub const OVERLAY_MUTEX_NAME: &str = "MidnightSentinel-Overlay-SingleInstance";

/// Window class shared by every overlay window, in whichever process/thread
/// created them. Used both to register the class and to find an
/// already-running overlay from a different process via [`FindWindowW`].
const WINDOW_CLASS_NAME: PCWSTR = w!("MidnightSentinelOverlayWindow");

/// Name registered with `RegisterWindowMessageW` so a dismiss request can be
/// posted across process boundaries: the OS guarantees the same name maps to
/// the same message ID in every process that registers it.
const DISMISS_MESSAGE_NAME: PCWSTR = w!("MidnightSentinel-Overlay-Dismiss");

thread_local! {
    /// Every overlay window created by this call, on this thread.
    static OVERLAY_WINDOWS: RefCell<Vec<HWND>> = const { RefCell::new(Vec::new()) };
}

/// Whether the overlay window class has been registered yet in this
/// process. Window classes are process-global (not per-thread), and
/// `run_overlay` can be called repeatedly in the same long-lived process
/// (e.g. the tray controller calling it once per double-click), so
/// registration must happen at most once or `RegisterClassW` fails.
static WINDOW_CLASS_REGISTERED: OnceLock<bool> = OnceLock::new();

/// Registered message ID used to ask a running overlay to dismiss itself.
/// Cached because `RegisterWindowMessageW` talks to the OS every call.
static DISMISS_MESSAGE_ID: OnceLock<u32> = OnceLock::new();

fn dismiss_message_id() -> u32 {
    *DISMISS_MESSAGE_ID.get_or_init(|| unsafe { RegisterWindowMessageW(DISMISS_MESSAGE_NAME) })
}

/// Creates the full-screen black overlay windows and runs a message loop
/// until they're all dismissed (double-click, or [`dismiss_running_overlay`]
/// called from elsewhere) or destroyed. Blocks the calling thread for the
/// duration; callers that need to keep doing other work (like the tray
/// controller) should call this from a dedicated thread.
pub fn run_overlay() {
    // SAFETY: all calls below are standard Win32 window creation/message-loop
    // calls, single-threaded, following the usual ownership rules (handles are
    // not used after the window that owns them is destroyed).
    unsafe {
        let hinstance = GetModuleHandleW(None).expect("GetModuleHandleW failed");

        let registered = *WINDOW_CLASS_REGISTERED.get_or_init(|| {
            let wc = WNDCLASSW {
                style: CS_DBLCLKS,
                lpfnWndProc: Some(wndproc),
                hInstance: hinstance.into(),
                hbrBackground: HBRUSH(GetStockObject(BLACK_BRUSH).0),
                lpszClassName: WINDOW_CLASS_NAME,
                ..Default::default()
            };
            RegisterClassW(&wc) != 0
        });
        if !registered {
            eprintln!("Failed to register overlay window class");
            return;
        }

        let monitor_rects = monitor_rects();
        if monitor_rects.is_empty() {
            eprintln!("No monitors detected; nothing to overlay.");
            return;
        }

        for rect in &monitor_rects {
            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST,
                WINDOW_CLASS_NAME,
                w!("Midnight Sentinel"),
                WS_POPUP,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                None,
                None,
                Some(hinstance.into()),
                None,
            )
            .expect("CreateWindowExW failed");

            OVERLAY_WINDOWS.with(|windows| windows.borrow_mut().push(hwnd));
            let _ = ShowWindow(hwnd, SW_SHOW);
        }

        let first_window = OVERLAY_WINDOWS.with(|windows| windows.borrow().first().copied());
        if let Some(first_window) = first_window {
            let _ = SetForegroundWindow(first_window);
        }

        hide_cursor();

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        restore_cursor();

        // Clear state so a second call on a reused thread starts clean.
        OVERLAY_WINDOWS.with(|windows| windows.borrow_mut().clear());
    }
}

/// Asks an already-running overlay to dismiss itself, exactly as if it had
/// been double-clicked — regardless of whether it's running in this
/// process (e.g. the tray controller's background thread) or a different
/// one (e.g. a previously-launched `MidSent_CLI`). Returns `true` if a
/// running overlay was found and asked to dismiss.
pub fn dismiss_running_overlay() -> bool {
    unsafe {
        let Ok(hwnd) = FindWindowW(WINDOW_CLASS_NAME, None) else {
            return false;
        };
        let _ = PostMessageW(Some(hwnd), dismiss_message_id(), WPARAM(0), LPARAM(0));
        true
    }
}

/// Collects the bounding rectangle of every active monitor.
fn monitor_rects() -> Vec<RECT> {
    let mut rects: Vec<RECT> = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(monitor_enum_proc),
            LPARAM(std::ptr::addr_of_mut!(rects) as isize),
        );
    }
    rects
}

unsafe extern "system" fn monitor_enum_proc(
    _hmonitor: HMONITOR,
    _hdc: HDC,
    rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let rects = &mut *(lparam.0 as *mut Vec<RECT>);
    rects.push(*rect);
    BOOL(1)
}

/// Hides the cursor reliably while it's over an overlay window. Relying only
/// on `ShowCursor` proved unreliable in practice (its display counter can be
/// out of sync with other applications), so the primary mechanism is
/// overriding `WM_SETCURSOR` below; `ShowCursor` is kept as a secondary,
/// belt-and-suspenders measure.
fn hide_cursor() {
    unsafe { while ShowCursor(false) >= 0 {} }
}

fn restore_cursor() {
    unsafe { while ShowCursor(true) < 0 {} }
}

/// Closes every tracked overlay window; each `WM_DESTROY` handler below
/// posts the quit message once the last one goes away.
fn close_all_overlays() {
    let handles = OVERLAY_WINDOWS.with(|windows| windows.borrow().clone());
    for hwnd in handles {
        unsafe {
            let _ = DestroyWindow(hwnd);
        }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if msg == dismiss_message_id() {
        close_all_overlays();
        return LRESULT(0);
    }

    match msg {
        WM_SETCURSOR => {
            // Force no cursor every time Windows would otherwise reset it
            // (e.g. on mouse move), rather than relying solely on the
            // global ShowCursor counter.
            let _ = SetCursor(None);
            LRESULT(1)
        }
        WM_LBUTTONDBLCLK | WM_RBUTTONDBLCLK | WM_MBUTTONDBLCLK => {
            close_all_overlays();
            LRESULT(0)
        }
        WM_DESTROY => {
            OVERLAY_WINDOWS.with(|windows| windows.borrow_mut().retain(|&h| h != hwnd));
            let remaining = OVERLAY_WINDOWS.with(|windows| windows.borrow().len());
            if remaining == 0 {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

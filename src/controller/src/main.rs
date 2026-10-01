//! Midnight Sentinel system tray controller (`midsent`).
//!
//! Hosts the tray icon and its menu (About / Exit) and runs the overlay
//! in-process on a background thread when the tray icon is double-clicked,
//! using the same `midsent-overlay` library as the standalone `midsentcli`
//! binary. Dialogs (About, duplicate-instance warning) are built with Slint.
#![windows_subsystem = "windows"]

slint::include_modules!();

use std::time::Duration;

use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};

const MUTEX_NAME: &str = "MidnightSentinel-Controller-SingleInstance";

fn main() {
    // `--run-now`: bypass the tray entirely and behave exactly like the
    // standalone `midsentcli` executable — toggle the overlay and exit,
    // via the same shared overlay mutex. Kept for backward compatibility
    // with the previous single-executable app's `--run-now` flag, since
    // existing third-party scripts/tools may already depend on it; new
    // integrations should prefer launching `midsentcli.exe` directly.
    if std::env::args().any(|a| a == "--run-now") {
        run_now();
        return;
    }

    match common::SingleInstance::acquire(MUTEX_NAME) {
        Ok(Some(guard)) => run_controller(guard),
        Ok(None) => show_already_running_warning(),
        Err(err) => eprintln!("Failed to acquire controller single-instance guard: {err}"),
    }
}

/// Toggles the overlay and exits, without ever creating a tray icon. See
/// the note on `--run-now` above.
fn run_now() {
    match common::SingleInstance::acquire(midsent_overlay::OVERLAY_MUTEX_NAME) {
        Ok(Some(_guard)) => midsent_overlay::run_overlay(),
        Ok(None) => {
            if !midsent_overlay::dismiss_running_overlay() {
                eprintln!(
                    "Midnight Sentinel overlay is already active, but no window was found to dismiss."
                );
            }
        }
        Err(err) => eprintln!("Failed to acquire overlay single-instance guard: {err}"),
    }
}

/// Shown when a second controller instance is launched. The current
/// (duplicate) process has nothing else to do but display this and exit.
fn show_already_running_warning() {
    let dialog = WarningDialog::new().expect("failed to create warning dialog");
    dialog.set_message("Another instance of Midnight Sentinel is already running.".into());
    center_on_screen(&dialog);

    let dialog_weak = dialog.as_weak();
    dialog.on_close_requested(move || {
        if let Some(dialog) = dialog_weak.upgrade() {
            let _ = dialog.hide();
        }
    });

    dialog.show().expect("failed to show warning dialog");
    slint::run_event_loop().expect("event loop failed");
}

fn run_controller(_guard: common::SingleInstance) {
    let about_item = MenuItem::new("About", true, None);
    let exit_item = MenuItem::new("Exit", true, None);
    let about_id = about_item.id().clone();
    let exit_id = exit_item.id().clone();

    let menu = Menu::new();
    menu.append(&about_item)
        .expect("failed to append About menu item");
    menu.append(&PredefinedMenuItem::separator())
        .expect("failed to append menu separator");
    menu.append(&exit_item)
        .expect("failed to append Exit menu item");

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(format!("Midnight Sentinel ({})", common::version_string()))
        .with_icon(load_icon(include_bytes!("../resources/systray.png")))
        .with_menu_on_left_click(false)
        .build()
        .expect("failed to create tray icon");

    // tray-icon needs a running Win32 message loop on this thread; Slint's
    // event loop already provides one, so we just poll its channels from a
    // repeating timer instead of standing up a second event loop.
    let poll_timer = slint::Timer::default();
    poll_timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(100),
        move || poll_tray_events(&about_id, &exit_id),
    );

    slint::run_event_loop_until_quit().expect("event loop failed");
}

fn poll_tray_events(about_id: &MenuId, exit_id: &MenuId) {
    if let Ok(TrayIconEvent::DoubleClick { .. }) = TrayIconEvent::receiver().try_recv() {
        launch_overlay();
    }

    if let Ok(event) = MenuEvent::receiver().try_recv() {
        if event.id == *about_id {
            show_about();
        } else if event.id == *exit_id {
            slint::quit_event_loop().expect("failed to quit event loop");
        }
    }
}

fn show_about() {
    let dialog = AboutDialog::new().expect("failed to create about dialog");
    dialog.set_version_text(common::version_string().into());
    center_on_screen(&dialog);

    let dialog_weak = dialog.as_weak();
    dialog.on_close_requested(move || {
        if let Some(dialog) = dialog_weak.upgrade() {
            let _ = dialog.hide();
        }
    });
    dialog.on_open_github(|| {
        if let Err(err) = open::that("https://github.com/SaltSpectre/MidnightSentinel") {
            eprintln!("Failed to open GitHub link: {err}");
        }
    });

    dialog.show().expect("failed to show about dialog");
}

/// Toggles the overlay: runs it in-process on a background thread if none
/// is active, or asks an already-running one to dismiss (whether it's ours
/// or a third-party-launched `midsentcli`'s) — same as a double-click.
fn launch_overlay() {
    match common::SingleInstance::acquire(midsent_overlay::OVERLAY_MUTEX_NAME) {
        Ok(Some(guard)) => {
            std::thread::spawn(move || {
                let _guard = guard;
                midsent_overlay::run_overlay();
            });
        }
        Ok(None) => {
            midsent_overlay::dismiss_running_overlay();
        }
        Err(err) => eprintln!("Failed to acquire overlay single-instance guard: {err}"),
    }
}

fn load_icon(bytes: &[u8]) -> Icon {
    let image = image::load_from_memory(bytes)
        .expect("failed to decode embedded icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    Icon::from_rgba(image.into_raw(), width, height).expect("failed to build tray icon")
}

/// Centers a dialog on the primary monitor using its declared logical size.
fn center_on_screen(dialog: &impl slint::ComponentHandle) {
    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    let window = dialog.window();
    let scale = window.scale_factor();
    let size = window.size();

    let (screen_width, screen_height) =
        unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };

    let x = ((screen_width as f32 - size.width as f32 * scale) / 2.0).max(0.0) as i32;
    let y = ((screen_height as f32 - size.height as f32 * scale) / 2.0).max(0.0) as i32;

    window.set_position(slint::WindowPosition::Physical(
        slint::PhysicalPosition::new(x, y),
    ));
}

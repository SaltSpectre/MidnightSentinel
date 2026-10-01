//! Small bits shared between the `controller` and `overlay` binaries.
//!
//! Deliberately minimal: each binary calls [`SingleInstance::acquire`] with its
//! own mutex name, so the two processes enforce single-instance independently
//! of one another.

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::System::Threading::{CreateMutexW, ReleaseMutex};

/// Holds the OS mutex handle that marks this process as "the" running
/// instance for a given name. Dropping it releases and closes the handle.
pub struct SingleInstance {
    handle: HANDLE,
}

// SAFETY: a Win32 HANDLE is just an opaque identifier; it's fine to move
// between threads as long as it's not used concurrently without synchronization,
// which we don't do here (it's only ever read on drop).
unsafe impl Send for SingleInstance {}

impl SingleInstance {
    /// Attempts to become the single running instance for `name`.
    ///
    /// Returns `Ok(Some(guard))` if this process is the first (and only)
    /// instance holding that name. Returns `Ok(None)` if another process
    /// already holds it. The name should be unique per-application so that
    /// unrelated processes (e.g. the overlay vs. the controller) don't
    /// interfere with each other.
    pub fn acquire(name: &str) -> windows::core::Result<Option<Self>> {
        let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let handle = unsafe { CreateMutexW(None, true, PCWSTR(wide_name.as_ptr()))? };
        let already_running = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;

        if already_running {
            unsafe {
                let _ = CloseHandle(handle);
            }
            Ok(None)
        } else {
            Ok(Some(Self { handle }))
        }
    }
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        unsafe {
            let _ = ReleaseMutex(self.handle);
            let _ = CloseHandle(self.handle);
        }
    }
}

/// Returns the application version as `v{CARGO_PKG_VERSION}`, e.g. `v0.1.0`.
pub fn version_string() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

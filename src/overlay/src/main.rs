//! Standalone `midsentcli` entry point: acquires the overlay single-
//! instance guard and runs the overlay, or — if one is already running,
//! whether launched by this binary, by `midsent --run-now`, or by the tray
//! controller — asks it to dismiss instead, so running this executable
//! always toggles the overlay. Designed to be launched directly — by a
//! third-party tool, a script, or manually — with no special flags required.
#![windows_subsystem = "windows"]

fn main() {
    match common::SingleInstance::acquire(midsent_overlay::OVERLAY_MUTEX_NAME) {
        Ok(Some(_guard)) => midsent_overlay::run_overlay(),
        Ok(None) => {
            // An overlay is already active somewhere; treat this launch as a
            // toggle and ask it to dismiss, same as a double-click.
            if !midsent_overlay::dismiss_running_overlay() {
                eprintln!(
                    "Midnight Sentinel overlay is already active, but no window was found to dismiss."
                );
            }
        }
        Err(err) => {
            eprintln!("Failed to acquire overlay single-instance guard: {err}");
        }
    }
}

//! Standalone `midsentcli` entry point: acquires the overlay single-
//! instance guard and runs the overlay, or — if one is already running,
//! whether launched by this binary, by `midsent --run-now`, or by the tray
//! controller — asks it to dismiss instead. Designed to be launched
//! directly — by a third-party tool, a script, or manually.
//!
//! Supports three optional, mutually exclusive flags; running with no flag
//! at all is identical to `--toggle`:
//! - `--toggle` (default): raise the overlay if it's not active, or dismiss
//!   it if it is. Prints nothing beyond error messages.
//! - `--start`: ensure the overlay is active. Prints the PID of the process
//!   that owns it (either this one, or an already-running one) to stdout so
//!   a script can monitor it, then — if this invocation is the one that
//!   just started it — blocks until it's dismissed.
//! - `--stop`: ensure the overlay is not active, dismissing it if it was.
//!   Prints human-readable feedback and exits with a status code (`0` if an
//!   overlay was found and dismissed, non-zero otherwise).
#![windows_subsystem = "windows"]

use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let start = args.iter().any(|a| a == "--start");
    let stop = args.iter().any(|a| a == "--stop");
    let toggle = args.iter().any(|a| a == "--toggle");

    match (start, stop, toggle) {
        (true, false, false) => start_overlay(),
        (false, true, false) => stop_overlay(),
        (false, false, _) => {
            // No recognized flag, or an explicit `--toggle`: both mean the
            // same thing.
            toggle_overlay();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Error: --start, --stop, and --toggle are mutually exclusive.");
            ExitCode::from(2)
        }
    }
}

/// `--toggle` (and the no-flag default): raise the overlay if it's not
/// active, or dismiss it (same as a double-click) if it is.
fn toggle_overlay() {
    match common::SingleInstance::acquire(midsent_overlay::OVERLAY_MUTEX_NAME) {
        Ok(Some(_guard)) => midsent_overlay::run_overlay(),
        Ok(None) => {
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

/// `--start`: ensure the overlay is active and print the owning PID.
fn start_overlay() -> ExitCode {
    match common::SingleInstance::acquire(midsent_overlay::OVERLAY_MUTEX_NAME) {
        Ok(Some(_guard)) => {
            print_pid(std::process::id());
            midsent_overlay::run_overlay();
            ExitCode::SUCCESS
        }
        Ok(None) => match midsent_overlay::running_overlay_pid() {
            Some(pid) => {
                print_pid(pid);
                ExitCode::SUCCESS
            }
            None => {
                eprintln!(
                    "Midnight Sentinel overlay is already active, but its owning process could not be identified."
                );
                ExitCode::FAILURE
            }
        },
        Err(err) => {
            eprintln!("Failed to acquire overlay single-instance guard: {err}");
            ExitCode::FAILURE
        }
    }
}

/// `--stop`: ensure the overlay is not active, with feedback and an exit code.
fn stop_overlay() -> ExitCode {
    if midsent_overlay::dismiss_running_overlay() {
        println!("Midnight Sentinel overlay dismissed.");
        ExitCode::SUCCESS
    } else {
        println!("Midnight Sentinel overlay was not active.");
        ExitCode::FAILURE
    }
}

/// Prints a PID on its own line and flushes immediately, so a script
/// capturing stdout sees it right away rather than waiting on buffering.
fn print_pid(pid: u32) {
    println!("{pid}");
    let _ = std::io::stdout().flush();
}

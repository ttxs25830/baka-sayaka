use signal_hook::consts::FORBIDDEN;
use signal_hook::low_level::register;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub fn init() {
    static INITED: AtomicBool = AtomicBool::new(false);
    if INITED.swap(true, Ordering::AcqRel) {
        return;
    }
    // 防杀：拦截停止运行的信号
    thread::spawn(move || {
        (1..32)
            .filter(|v| !FORBIDDEN.contains(&v))
            .for_each(|v| unsafe {
                register(v, || ()).expect(format!("Err reg {}", v).as_str());
            })
    });
}

use std::thread;
use std::time::Duration;

use rodio::Source;
use utils::PlayHandle;
mod assets;
#[cfg(feature = "hide-window")]
#[cfg(windows)]
mod hide_windows;
#[cfg(feature = "unkill-advance")]
#[cfg(windows)]
mod unkill_advance;
#[cfg(feature = "unkill-signal")]
mod unkill_signal;
mod utils;
fn main() {
    #[cfg(feature = "hide-window")]
    #[cfg(windows)]
    hide_windows::init();
    #[cfg(feature = "unkill-signal")]
    unkill_signal::init();
    #[cfg(feature = "unkill-advance")]
    #[cfg(windows)]
    unkill_advance::init();

    let device = PlayHandle::try_default().unwrap();
    let decretum = assets::get_source();
    device
        .play_raw(decretum.repeat_infinite().convert_samples())
        .unwrap();
    thread::sleep(Duration::from_secs(114514));
}

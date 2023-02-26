use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, ShowWindow, SW_HIDE};
pub fn init() {
    unsafe {
        let window_handle = GetForegroundWindow();
        ShowWindow(window_handle, SW_HIDE);
    }
}

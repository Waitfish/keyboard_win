use keyboard_win::keyboard;
use std::ffi::CString;
use winapi::um::winuser::VK_F5;
fn main() {
    let hwnd = keyboard::get_hwnd_by_win_title(CString::new("123.txt - Notepad").unwrap());
    keyboard::key_down(hwnd, VK_F5 as usize);
}
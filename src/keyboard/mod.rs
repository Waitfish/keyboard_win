use std::ffi::CString;
use std::thread::sleep;
use winapi::shared::windef;
use winapi::um::winuser::{FindWindowA, PostMessageA,  VK_F5, WM_KEYDOWN, WM_KEYUP};

pub fn get_hwnd_by_win_title(title: CString) -> windef::HWND {
    unsafe { FindWindowA(std::ptr::null(), title.as_ptr()) }
}

pub fn key_down(hwnd: windef::HWND, key: usize) {
    unsafe {
        PostMessageA(hwnd, WM_KEYDOWN, key, 0);
    }
}

pub fn key_up(hwnd:windef::HWND,key:usize){
    unsafe {
        PostMessageA(hwnd, WM_KEYUP, key, 0);
    }
}

pub fn key_press(hwnd: windef::HWND, key: usize) {
    unsafe {
        PostMessageA(hwnd, WM_KEYDOWN, key, 0);
        sleep(std::time::Duration::from_millis(100));
        PostMessageA(hwnd, WM_KEYUP, key, 0);
    }
}



#[cfg(test)]
mod keyboard_tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn get_hwnd_by_title() {
        let hwnd = get_hwnd_by_win_title(CString::new("123.txt - Notepad").unwrap());
        print!("{:?}", hwnd);
        assert_eq!(hwnd, std::ptr::null_mut());
    }

    #[test]
    fn test_key_down() {
        let hwnd = get_hwnd_by_win_title(CString::new("123.txt - Notepad").unwrap());
        print!("{:?}", hwnd);
        key_down(hwnd,VK_F5 as usize);
    }
}

use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindow, GetWindowTextW, GW_CHILD};

pub fn get_explorer_path() -> OsString {
    unsafe {
        // Try to find an Explorer window
        let hwnd = FindWindowW(w!("CabinetWClass"), ptr::null())
            .or_else(|| FindWindowW(w!("ExploreWClass"), ptr::null()));
        
        if hwnd == 0 {
            return OsString::new();
        }

        // Navigate through the window hierarchy
        let rebar = GetWindow(hwnd, GW_CHILD);
        if rebar == 0 {
            return OsString::new();
        }

        let address_band_root = GetWindow(rebar, GW_CHILD);
        if address_band_root == 0 {
            return OsString::new();
        }

        let toolbar = GetWindow(address_band_root, GW_CHILD);
        if toolbar == 0 {
            return OsString::new();
        }

        // Get the window text
        let mut path = [0u16; 260]; // MAX_PATH
        let len = GetWindowTextW(toolbar, path.as_mut_ptr(), path.len() as i32);
        if len == 0 {
            return OsString::new();
        }

        let path_str = OsString::from_wide(&path[..len as usize]);
        
        // Remove the "Address: " prefix if present
        if let Some(path_str) = path_str.to_string_lossy().strip_prefix("Address: ") {
            OsString::from(path_str)
        } else {
            path_str
        }
    }
}
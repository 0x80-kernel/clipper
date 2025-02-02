/// src/lib.rs
/// clipboard

/// winapi modules
pub mod clipboard {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use winapi::ctypes::c_void;
    use winapi::shared::minwindef::HGLOBAL;
    use winapi::shared::ntdef::NULL;
    use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
    use winapi::um::winuser::{
        CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData, CF_TEXT,
    };

    /// Opens the clipboard
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Returns Ok() if the clipboard was able to open.
    /// * `Err(&str)` - Returns Err() if the clipboard could not be opened.
    ///
    pub fn open_clipboard() -> Result<(), &'static str> {
        unsafe {
            if OpenClipboard(null_mut()) == 0 {
                Err("Failed to open the clipboard")
            } else {
                Ok(())
            }
        }
    }

    /// Closes the clipboard
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Returns Ok() if the clipboard was closed.
    /// * `Err(&str)` - Returns Err() if the clipboard could not be closed.
    ///
    pub fn close_clipboard() -> Result<(), &'static str> {
        unsafe {
            if CloseClipboard() == 0 {
                Err("Failed to close the clipboard")
            } else {
                Ok(())
            }
        }
    }

    /// Gets clipboard text and returns it
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - If the text was successfully retrieved.
    /// * `Err(&str)` - If the text could not be retrieved.
    ///
    pub fn get_clipboard_text() -> Result<String, &'static str> {
        unsafe {
            let clipboard_data: *mut c_void = GetClipboardData(CF_TEXT);
            if clipboard_data.is_null() {
                close_clipboard()?;
                return Err("Failed to get clipboard text");
            }
            let locked_data: *const u8 = GlobalLock(clipboard_data) as *const u8;
            if locked_data.is_null() {
                close_clipboard()?;
                return Err("Failed to lock clipboard data");
            }
            let c_str: &std::ffi::CStr = std::ffi::CStr::from_ptr(locked_data as *const i8);
            let text: String = c_str
                .to_str()
                .map_err(|_| "Failed to convert clipboard data to string")?
                .to_owned();
            GlobalUnlock(clipboard_data);
            Ok(text)
        }
    }

    /// Changes the current clipboard text to the one given
    ///
    /// # Parameters
    ///
    /// * `text`- A string slice with the text replacing the clipboard.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the clipboard text was set successfully.
    /// * `Err(&str)` - If there was an error setting the clipboard text.
    ///
    pub fn set_clipboard_text(text: &str) -> Result<(), &'static str> {
        let text_cstring: CString =
            CString::new(text).map_err(|_| "Failed to create CSTRING from text (null byte)")?;
        let text_len = text_cstring.as_bytes_with_nul().len();
        let h_mem: *mut c_void = unsafe { GlobalAlloc(GMEM_MOVEABLE, text_len) } as *mut c_void;
        if h_mem.is_null() {
            close_clipboard()?;
            return Err("Failed to allocate memory for clipboard data");
        }
        let h_mem_text: *mut u64 = unsafe { GlobalLock(h_mem) as *mut u64 };
        if h_mem_text.is_null() {
            unsafe { GlobalFree(h_mem) };
            close_clipboard()?;
            return Err("Failed to lock memory for clipboard data");
        }
        unsafe {
            let bytes: &[u8] = text_cstring.as_bytes_with_nul();
            let src: *const u64 = bytes.as_ptr() as *const u64;
            let dst: *mut u64 = h_mem_text as *mut u64;
            let len: usize = bytes.len() / 8;
            for i in 0..len {
                *dst.offset(i as isize) = *src.offset(i as isize);
            }
            // Remaining bytes if needed
            let remaining: usize = bytes.len() % 8;
            if remaining > 0 {
                let src_bytes: *const u8 = src.offset(len as isize) as *const u8;
                let dst_bytes: *mut u8 = dst.offset(len as isize) as *mut u8;
                for i in 0..remaining {
                    *dst_bytes.offset(i as isize) = *src_bytes.offset(i as isize);
                }
            }
        }
        unsafe {
            GlobalUnlock(h_mem);
            EmptyClipboard();
            if SetClipboardData(CF_TEXT, h_mem as HGLOBAL) == NULL {
                GlobalFree(h_mem);
                close_clipboard()?;
                return Err("Failed to set clipboard data");
            }
        }
        Ok(())
    }
}

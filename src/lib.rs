/// src/lib.rs
/// clipboard

/// winapi modules
pub mod clipboard {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use winapi::ctypes::c_void;
    use winapi::shared::minwindef::HGLOBAL;
    use winapi::shared::ntdef::NULL;
    use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
    use winapi::um::winuser::{
        CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData, CF_TEXT,
    };

    /// Opens the clipboard
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Returns Ok() if the clipboard was able to open
    /// * `Err(&str)` - Returns Err() if the clipboard could not be opened
    ///
    pub fn open_clipboard() -> Result<(), &'static str> {
        unsafe {
            if OpenClipboard(null_mut()) == 0 {
                return Err("Failed to open the clipboard");
            } else {
                return Ok(());
            }
        }
    }

    /// Closes the clipboard
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Returns Ok() if the clipboard was closed
    /// * `Err(&str)` - Returns Err() if the clipboard could not be closed
    ///
    pub fn close_clipboard() -> Result<(), &'static str> {
        unsafe {
            if CloseClipboard() == 0 {
                return Err("Failed to close the clipboard");
            } else {
                return Ok(());
            }
        }
    }

    /// Checks if the variable given is null
    ///
    /// # Arguments
    ///
    /// * `val` - A mutable c_void (C type void)
    /// * `err` - A string slice for the error message
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value given is not null
    /// * `Err(&str)` - If the value given is null
    ///
    fn check_null(val: *mut c_void, err: &str) -> Result<(), &str> {
        if val.is_null() {
            close_clipboard()?;
            return Err(err);
        }
        return Ok(());
    }

    /// Gets clipboard text and returns it
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - If the text was successfully retrieved
    /// * `Err(&str)` - If the text could not be retrieved
    ///
    pub fn get_clipboard_text() -> Result<String, &'static str> {
        unsafe {
            let clipboard_data: *mut c_void = GetClipboardData(CF_TEXT);
            check_null(clipboard_data, "Failed to get clipboard text")?;
            let c_str: &std::ffi::CStr = std::ffi::CStr::from_ptr(clipboard_data as *const i8);
            let text: String = c_str
                .to_str()
                .map_err(|_| "Failed to convert clipboard data to string")?
                .to_owned();
            return Ok(text);
        }
    }

    /// Changes the current clipboard text to the one given
    ///
    /// # Arguments
    ///
    /// * `text`- A string slice with the text replacing the clipboard
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the clipboard text was set successfully
    /// * `Err(&str)` - If there was an error setting the clipboard text
    ///
    pub fn set_clipboard_text(text: &str) -> Result<(), &'static str> {
        let text_cstring: CString =
            CString::new(text).map_err(|_| "Failed to create CSTRING from text (null byte)")?;
        let text_len: usize = text_cstring.as_bytes_with_nul().len();
        let h_heap: *mut c_void = unsafe { GetProcessHeap() };
        check_null(h_heap, "Failed to get process heap")?;
        let h_mem: *mut c_void = unsafe { HeapAlloc(h_heap, 0, text_len) } as *mut c_void;
        check_null(h_mem, "Failed to allocate memory for clipboard data")?;
        unsafe {
            let h_mem_text: *mut u8 = h_mem as *mut u8;
            let bytes: &[u8] = text_cstring.as_bytes_with_nul();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), h_mem_text, text_len);
        }
        unsafe {
            EmptyClipboard();
            if SetClipboardData(CF_TEXT, h_mem as HGLOBAL) == NULL {
                HeapFree(h_heap, 0, h_mem);
                close_clipboard()?;
                return Err("Failed to set clipboard data");
            }
        }
        return Ok(());
    }
}

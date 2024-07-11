use std::io::{Read, Write};
use std::time::Duration;

use crate::{Error, ErrorKind, Result};
use crate::{Port, Serial};
use crate::windows::dcb::*;
use crate::windows::ffi::*;

#[derive(Debug, Clone)]
pub struct COMPort {
    handle: HANDLE,
    timeout: Duration,
}

impl COMPort {
    pub fn open(serial: &Serial) -> Result<COMPort> {
        let name = &serial.path;

        // \\.\ + 0 = 5
        let mut path = Vec::<u16>::with_capacity(5 + name.len());

        path.extend(r"\\.\".encode_utf16());
        path.extend(name.encode_utf16());
        path.push(0);

        let handle = unsafe {
            CreateFileW(
                path.as_ptr() as LPCWSTR,
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                0 as HANDLE,
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::new(ErrorKind::Unknown, "Invalid handle"));
        }

        let mut com = Self::open_from_handle(handle);
        com.timeout = serial.timeout;
        set_timeouts(handle, serial.timeout);

        let mut dcb = get_dcb(handle)?;
        set_baud_rate(&mut dcb, serial.baud_rate);
        set_data_bits(&mut dcb, serial.data_bit);
        set_stop_bits(&mut dcb, serial.stop_bit);
        set_parity(&mut dcb, serial.parity);
        set_flow_control(&mut dcb, serial.flow_ctrl);

        let _ = set_dcb(handle, dcb);

        Ok(com)
    }

    fn open_from_handle(handle: HANDLE) -> Self {
        Self {
            handle,
            timeout: Duration::from_secs(0),
        }
    }
}

unsafe impl Send for COMPort {}

impl Read for COMPort {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            let mut len: DWORD = 0;
            let res = ReadFile(
                self.handle,
                buf.as_mut_ptr() as LPVOID,
                buf.len() as DWORD,
                &mut len as LPDWORD,
                std::ptr::null_mut(),
            );
            match res {
                0 => {
                    Err(std::io::Error::last_os_error())
                }
                _ => {
                    if len != 0 {
                        Ok(len as usize)
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out"))
                    }
                }
            }
        }
    }
}

impl Write for COMPort {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            let mut len: DWORD = 0;
            let res = WriteFile(self.handle, buf.as_ptr() as LPVOID, buf.len() as DWORD, &mut len, std::ptr::null_mut());
            match res {
                0 => {
                    Err(std::io::Error::last_os_error())
                }
                _ => {
                    Ok(len as usize)
                }
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unsafe {
            let res = FlushFileBuffers(self.handle);
            match res {
                0 => {
                    Err(std::io::Error::last_os_error())
                }
                _ => {
                    Ok(())
                }
            }
        }
    }
}

impl Port for COMPort {}

impl Drop for COMPort {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

#![cfg(target_os = "windows")]

use std::env::current_exe;
use std::os::windows::process::CommandExt;
use std::ptr::null_mut;

use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_QUERY};

pub struct WindowsAPI;

impl WindowsAPI {
    pub fn check_admin() -> bool {
        unsafe {
            let mut token_handle: HANDLE = null_mut();
            let mut token_elevation: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let token_elevation_size: u32 = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
                return false;
            }

            let mut return_length: u32 = 0;
            let result: i32 = GetTokenInformation(
                token_handle,
                TokenElevation,
                &mut token_elevation as *mut _ as *mut _,
                token_elevation_size,
                &mut return_length,
            );

            CloseHandle(token_handle);

            if result == 0 {
                return false;
            }

            token_elevation.TokenIsElevated != 0
        }
    }

    pub fn restart_as_admin() {
        std::process::Command::new("powershell")
            .arg("-Command")
            .arg("Start-Process")
            .arg(format!("\"{}\"", current_exe().unwrap().to_str().unwrap()))
            .arg("-Verb")
            .arg("runAs")
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .expect("Error al reiniciar el programa como administrador");

        std::process::exit(0);
    }
}

use std::env::current_exe;
use std::ptr::null_mut;

use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_QUERY};

use crate::logger::Logger;

pub struct WindowsAPI;

impl WindowsAPI {
    fn has_admin_rights() -> bool {
        unsafe {
            let mut token_handle: HANDLE = null_mut();
            let mut token_elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let token_elevation_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
                return false;
            }

            let mut return_length: u32 = 0;
            let result = GetTokenInformation(
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

    pub fn check_admin() {
        if Self::has_admin_rights() {
            return;
        };

        Logger::info("Reiniciando con permisos de administrador para evitar problemas...");
        std::process::Command::new("powershell")
            .arg("-Command")
            .arg("Start-Process")
            .arg(format!("\"{}\"", current_exe().unwrap().to_str().unwrap()))
            .arg("-Verb")
            .arg("runAs")
            .spawn()
            .expect("Error al reiniciar el programa como administrador");

        std::process::exit(0);
    }
}

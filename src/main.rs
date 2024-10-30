#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use shadow_rs::shadow;
use std::env;
use std::process::exit;

use windows::{
    core::*,
    Win32::{
        Foundation::{CloseHandle, GetLastError, HWND, WAIT_FAILED},
        System::Threading::{
            CreateProcessA, WaitForSingleObject, INFINITE, PROCESS_CREATION_FLAGS,
            PROCESS_INFORMATION, STARTUPINFOA,
        },
        UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_OK},
    },
};

shadow!(build);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let message = "Usage: <command> [arguments]";
        let title = "Error";
        unsafe {
            MessageBoxA(
                HWND(std::ptr::null_mut()),
                PCSTR(message.as_ptr()),
                PCSTR(title.as_ptr()),
                MB_OK | MB_ICONERROR,
            );
        }
        exit(1);
    }

    let command = &args[1];
    let parameters: String = args[2..].join(" ");
    let mut full_command = format!("{} {}", command, parameters);

    let si = STARTUPINFOA {
        dwFlags: windows::Win32::System::Threading::STARTF_USESHOWWINDOW,
        wShowWindow: 0,
        ..Default::default()
    };

    let mut pi = PROCESS_INFORMATION::default();

    let success = unsafe {
        CreateProcessA(
            None,
            PSTR(full_command.as_mut_ptr()),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &si,
            &mut pi,
        )
    };

    match success {
        Ok(_) => {
            let wait_result = unsafe { WaitForSingleObject(pi.hProcess, INFINITE) };
            if wait_result == WAIT_FAILED {
                let error_code = unsafe { GetLastError().0 };
                let error_msg = format!("WaitForSingleObject failed: {}", error_code);
                let title = "Error";
                unsafe {
                    MessageBoxA(
                        HWND(std::ptr::null_mut()),
                        PCSTR(error_msg.as_ptr()),
                        PCSTR(title.as_ptr()),
                        MB_OK | MB_ICONERROR,
                    );
                }
                exit(1);
            }
        }
        Err(_) => {
            let error_code = unsafe { GetLastError().0 };
            let error_msg = format!("CreateProcess failed: {}", error_code);
            let title = "Error";
            unsafe {
                MessageBoxA(
                    HWND(std::ptr::null_mut()),
                    PCSTR(error_msg.as_ptr()),
                    PCSTR(title.as_ptr()),
                    MB_OK | MB_ICONERROR,
                );
            }
            exit(1);
        }
    }

    unsafe {
        if let Err(e) = CloseHandle(pi.hProcess) {
            let error_msg = format!("Failed to close process handle: {:?}", e);
            let title = "Error";
            MessageBoxA(
                HWND(std::ptr::null_mut()),
                PCSTR(error_msg.as_ptr()),
                PCSTR(title.as_ptr()),
                MB_OK | MB_ICONERROR,
            );
            exit(1);
        }
        if let Err(e) = CloseHandle(pi.hThread) {
            let error_msg = format!("Failed to close thread handle: {:?}", e);
            let title = "Error";
            MessageBoxA(
                HWND(std::ptr::null_mut()),
                PCSTR(error_msg.as_ptr()),
                PCSTR(title.as_ptr()),
                MB_OK | MB_ICONERROR,
            );
            exit(1);
        }
    }
}

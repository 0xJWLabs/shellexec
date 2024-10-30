#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
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

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn display_message_box(message: &str, err: Option<bool>) {
    unsafe {
        MessageBoxA(
            HWND(std::ptr::null_mut()),
            PCSTR(format!("{}\0", message).as_ptr()),
            PCSTR(format!("{} v{}\0", NAME, VERSION).as_ptr()),
            if err.unwrap_or(true) { MB_ICONERROR | MB_OK } else { MB_OK }, // Combine both flags
        );
    }
}

fn display_usage() {
    let usage = format!(
        "Usage: {} <command> [arguments]\n\n\
        Example:\n\
        {} pwsh -Command echo Hello, World!"
    , NAME, NAME);
    display_message_box(&usage, None);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        display_usage(); 
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
                display_message_box(&error_msg, None);
                exit(1);
            }
        }
        Err(_) => {
            let error_code = unsafe { GetLastError().0 };
            let error_msg = format!("CreateProcess failed: {}", error_code);
            display_message_box(&error_msg, None);
            exit(1);
        }
    }

    unsafe {
        if let Err(e) = CloseHandle(pi.hProcess) {
            let error_msg = format!("Failed to close process handle: {:?}", e);
            display_message_box(&error_msg, None);
            exit(1);
        }
        if let Err(e) = CloseHandle(pi.hThread) {
            let error_msg = format!("Failed to close thread handle: {:?}", e);
            display_message_box(&error_msg, None);
            exit(1);
        }
    }
}
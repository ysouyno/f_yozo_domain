#![windows_subsystem = "windows"]

extern crate winrt_notification;
use std::{os::windows::process::CommandExt, process};
use winrt_notification::{Duration, Sound, Toast};

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn check_ping() -> Result<(), std::io::Error> {
    let child = process::Command::new("cmd")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/c", "call", "ping", "www.baidu.com"])
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed to exec ping command.");

    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Network is unreachable.",
        ))
    }
}

fn main() {
    let mut count = 1;

    while count < 100 {
        if let Err(e) = check_ping() {
            Toast::new(Toast::POWERSHELL_APP_ID)
                .title(e.to_string().as_str())
                .text1(format!("Run logon.exe now. times: {}", count).as_str())
                .sound(Some(Sound::SMS))
                .duration(Duration::Short)
                .show()
                .expect("unable to toast");

            process::Command::new("cmd")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/c", "call", "D:\\dnld\\logon.exe"])
                .stdout(process::Stdio::piped())
                .spawn()
                .expect("Failed to run logon.exe");

            count += 1;
        }

        std::thread::sleep(std::time::Duration::from_secs(9));
    }
}

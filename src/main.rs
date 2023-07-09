use std::{io::BufRead, process};

fn check_ping() -> Result<(), std::io::Error> {
    let mut child = process::Command::new("cmd")
        .args(["/c", "call", "ping", "www.baidu.com"])
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed to exec ping");

    let stdout = child.stdout.as_mut().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not capture standard output.",
        )
    })?;

    let reader = std::io::BufReader::new(stdout);
    reader
        .lines()
        .map_while(Result::ok)
        .for_each(|line| println!("{}", line));

    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        Ok(())
    } else {
        println!("{}", output.status);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Network disconnected.",
        ))
    }
}

fn main() {
    let mut count = 1;

    loop {
        match check_ping() {
            Err(e) => {
                count += 1;
                println!("err: {}, Call logon.exe now, times: {}", e, count);

                process::Command::new("cmd")
                    .args(["/c", "call", "D:\\dnld\\logon.exe"])
                    .stdout(process::Stdio::piped())
                    .spawn()
                    .expect("Failed to exec logon.exe")
                    .wait_with_output()
                    .unwrap();
            }
            _ => {
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        }
    }
}

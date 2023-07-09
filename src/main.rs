use std::process;

fn check_ping() -> Result<(), std::io::Error> {
    let child = process::Command::new("cmd")
        .args(["/c", "call", "ping", "www.baidu.com"])
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed to exec ping command.");

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
        if let Err(e) = check_ping() {
            println!("err: {} Call logon.exe now, times: {}", e, count);

            process::Command::new("cmd")
                .args(["/c", "call", "D:\\dnld\\logon.exe"])
                .stdout(process::Stdio::piped())
                .spawn()
                .expect("Failed to exec logon.exe");

            count += 1;
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}

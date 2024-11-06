use serial2::SerialPort;

#[cfg(target_os = "linux")]
static COM_PATH: &str = "/dev/ttyUSB1";
#[cfg(target_os = "windows")]
static COM_PATH: &str = "COM6";
use std::time::Duration;
// A one second timeout
const TIME_OUT: Duration = Duration::from_millis(1000);

pub fn open() -> std::io::Result<SerialPort> {
    let mut port = SerialPort::open(COM_PATH, 115200)?;
    // Needed for windows, but should not hurt on Linux
    port.set_dtr(true)?;
    port.set_rts(true)?;
    port.set_write_timeout(TIME_OUT)?;
    port.set_read_timeout(TIME_OUT)?;

    Ok(port)
}

use rtmt_host::ncobs::decode;
use rtmt_host::rtmt::RtmtFrame;

fn main() {
    let port = open().unwrap();

    let mut buffer = [0; 256];
    let mut coded = vec![];
    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                for b in &buffer[0..n] {
                    // if we've received a sentinel (0), decode the "top" frame
                    if *b == 0 {
                        coded.push(*b);
                        let decoded_frame = decode(&mut coded);
                        let frame = RtmtFrame::try_from_bytes(&decoded_frame).unwrap();
                        println!("{}", frame);
                    } else {
                        coded.push(*b);
                    }
                }
            }
            Err(_) => {}
        }
    }
}

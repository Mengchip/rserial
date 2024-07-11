use std::io::{Read, Write};
use std::time::Duration;

use rserial::Serial;

fn main() {
    let mut serial = Serial::new("COM3", 115200).open().unwrap();
    loop {
        let _ = serial.write(b"AT\r\n");
        let mut buf: Vec<u8> = vec![0; 512];
        let len = serial.read(&mut buf).unwrap();
        unsafe {
            buf.set_len(len);
        }
        println!("Buff: {:?}, Len:{:?}", String::from_utf8(buf).unwrap(), len);
        std::thread::sleep(Duration::from_secs(2));
    }
}

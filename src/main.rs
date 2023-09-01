use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    let alpha: [char; 5] = ['.', '-', '=', '#', '@'];
    let mut counter = 0;
    let ten_millis = time::Duration::from_millis(100);
    let mut framerate = 0;
    while(true){
        let buf: [u8; 3600] = [alpha[counter] as u8; 3600];
        let ar:String = String::from_utf8_lossy(&buf).into_owned();
        write!(handle, "{:}", ar); // add `?` if you care about errors here
        //println!("{:}", ar);
        counter = (counter + 1) % 5;
        thread::sleep(ten_millis);
    }
}

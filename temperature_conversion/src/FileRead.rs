use std::io::Seek;
use std::io::SeekFrom;
use std::fs::File;
use std::io::Read;
use std::{thread, time};

fn main() {

    //they are doing exactly what im trying to do
    //https://github.com/uutils/coreutils/blob/master/src/tail/tail.rs

    let mut new_current_position = 10;

    while true {
        let mut f = File::open("/home/sugs/Documents/xi-gtk/TailLogTest").expect("file not found");
        let end_position = f.seek(SeekFrom::End(0)).unwrap();

        if(end_position > new_current_position) {
            let diff = end_position - new_current_position;
            println!("Diff between End position {} and current_stop_postion is {:?}", end_position, diff);

            let mut buf = vec![0; diff as usize];
            let pos = f.seek(SeekFrom::Current(-(buf.len() as i64))).unwrap();
            f.read_exact(&mut buf).unwrap();
            println!("{} {:?}", pos, String::from_utf8(buf).unwrap());

            new_current_position = end_position;
            println!("new_current_position at end is {}", new_current_position);

            let one_sec = time::Duration::from_millis(1000);
            thread::sleep(one_sec);
        }
    }
}
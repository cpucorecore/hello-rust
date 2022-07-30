use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

const BUF_SIZE: usize = 1024*1024;
const SRC_FILE: &str = "/dev/urandom";
const DEST_DIR: &str = "/home/sky/data/test";

fn copy<R, W>(r: &mut R, w: &mut W, size: u64) where R: Read, W: Write {
    let mut buf = vec![0u8; BUF_SIZE];
    let mut written = 0u64;

    loop {
        let len = match r.read(&mut buf) {
            Ok(0) => continue,
            Ok(len) => len,
            Err(e) => {
                println!("read err {}", e);
                break;
            }
        };
        w.write_all(&mut buf).unwrap();

        written += len as u64;
        if written >= size {
            break;
        }
    }
}

pub fn gen_random_files(file_size: u64, file_number: u64) {
    let mut jhs = vec![];

    for i in 1..file_number + 1 {
        let dir = PathBuf::from_str(DEST_DIR).unwrap();
        let path = dir.join(Path::new(i.to_string().as_str()));

        jhs.push(std::thread::spawn(move || {
            let mut read_file = File::open(SRC_FILE).unwrap();
            let mut write_file = File::create(path).unwrap();
            copy(&mut read_file, &mut write_file, file_size);
        }));
    }

    let _: Vec<()> = jhs.into_iter().map(|jh| {
        jh.join().unwrap();
    }).collect();
}

#[test]
fn test_gen_file() {
    gen_random_files(1024*1024*10, 100);
}

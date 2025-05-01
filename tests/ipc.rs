use shared_memory::ShmemConf;
use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let mut s = ShmemConf::new().size(1024).create().unwrap();
            let bytes = unsafe { s.as_slice_mut() };
            bytes[0..4].copy_from_slice(b"Ping");
            let mut child = Command::new(args[0].as_str())
                .arg(s.get_os_id())
                .spawn()
                .unwrap();
            assert_eq!(child.wait().unwrap().code(), Some(0));
            let bytes = unsafe { s.as_slice() };
            assert_eq!(&bytes[0..4], b"PONG");
        }
        2 => {
            let mut s = ShmemConf::new().os_id(args[1].as_str()).open().unwrap();
            let bytes = unsafe { s.as_slice_mut() };
            assert_eq!(&bytes[0..4], b"Ping");
            bytes[0..4].copy_from_slice(b"PONG");
        }
        _ => {
            panic!("Invalid arguments");
        }
    }
}

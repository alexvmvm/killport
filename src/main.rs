extern crate netstat;
extern crate clap;

use netstat::*;
use sysinfo::{ProcessExt, SystemExt, Signal, System};
use clap::{Arg, App};

fn get_process_pid_from_port(port: u16) -> Option<Vec<u32>> {
    
    let sockets_info = get_sockets_info(
    AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6, 
    ProtocolFlags::TCP | ProtocolFlags::UDP).unwrap();

    for socket in sockets_info {
        match socket.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                if tcp_si.local_port == port {
                    return Some(socket.associated_pids);
                }
            },
            ProtocolSocketInfo::Udp(udp_si) => {
                if udp_si.local_port == port {
                    return Some(socket.associated_pids)
                }
            }
        }
    }

    None
}

fn kill_process_with_pid(pid: usize) {
    let mut system = System::new();
    system.refresh_all();
    match system.get_process(pid) {
        Some(process) => {
            if process.kill(Signal::Kill) {
                println!("killed process {} with pid {}", process.name(), pid);
            }
        },
        None => println!("failed to find process")
    }
}

fn kill_process_with_port(port: u16) {
       match get_process_pid_from_port(port) {
        Some(pids) => {
            for pid in pids {
                kill_process_with_pid(pid as usize);
            }
        },
        None => ()
    }
}

fn main() {
    
    let matches = App::new("PortKill")
        .version("0.1.0")
        .author("Alex Mulford <alex.mulford@gmail.com>")
        .about("Kill Process on Port")
        .arg(Arg::with_name("port")
                .short("p")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("port to find process on and kill"))
        .get_matches();
    


    match matches.value_of("port") {
        Some(port) => {
            match port.parse::<u16>() {
                Ok(port_u16) => {
                    println!("killing processes on port {}..", port_u16);
                    kill_process_with_port(port_u16);
                    println!("done")
                },
                _ => println!("invalid port supplied, {}", port)
            }
        },
        _ => println!("no port provided")
    }

}

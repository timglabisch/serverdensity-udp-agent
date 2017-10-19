use std::net::UdpSocket;
use byteorder::{BigEndian};
use byteorder::ByteOrder;
use config::Config;
use ::Metric;
use std::sync::mpsc::Sender;


pub struct UdpServer;

impl UdpServer {
    pub fn new() -> Self {
        UdpServer {}
    }

    pub fn run(&self, config: &Config, sender: Sender<Metric>)
    {
            let mut socket = match UdpSocket::bind(&config.bind) {
            Ok(s) => s,
            Err(_) => {
                println!("could not listen, may someone is already listen on this port or the address is invalid?");
                return;
            }
        };

        loop {
            
            match self.read(&mut socket) {
                Err(_) => {
                    println!("could not read from socket.\n");
                    continue;
                },
                Ok(m) => {
                    match sender.send(m) {
                        Ok(_) => {},
                        Err(_) => {
                            println!("could not recv. metric");
                        }
                    }
                }
            }
        }
    }

    fn read(&self, socket : &mut UdpSocket) -> Result<Metric, String>
    {
        let mut buf = [0; 300];
        let (amt, _) = try!(socket.recv_from(&mut buf).or_else(|_|Err("Could recv from Socket.".to_string())));

        if amt <= 6 {
            return Err("UDP Package size is to small.".to_string());
        }

        let metric_type = BigEndian::read_u16(&buf[0..2]);

        if metric_type != 42 {
            return Err("unsupported metric type".to_string());
        }

        let count = BigEndian::read_i32(&buf[2..6]);
        let name = String::from_utf8_lossy(&buf[6..amt]).to_string().replace("\"", "");

        Ok(Metric {
            count: count,
            name
        })
    }
}
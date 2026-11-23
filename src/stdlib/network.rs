use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{self, Read, Write};

pub struct D0Network;

impl D0Network {
    pub fn listen(bind_addr: &str) -> io::Result<TcpListener> {
        TcpListener::bind(bind_addr)
    }

    pub fn connect(remote_addr: impl ToSocketAddrs) -> io::Result<TcpStream> {
        TcpStream::connect(remote_addr)
    }

    pub fn send_bytes(stream: &mut TcpStream, data: &[u8]) -> io::Result<usize> {
        stream.write(data)
    }

    pub fn read_bytes(stream: &mut TcpStream, buffer: &mut [u8]) -> io::Result<usize> {
        stream.read(buffer)
    }
}

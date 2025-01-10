use std::net::{TcpListener, TcpStream, UdpSocket, ToSocketAddrs};
use log::{error, info};
use crate::error::NetOpResult;

fn new_tcp_listener<T: ToSocketAddrs>(inetaddr: &T) -> NetOpResult<TcpListener> {
	match TcpListener::bind(inetaddr) {
		Ok(listener) => {
			info!("Created a new TcpListener!");
			NetOpResult::Success(listener)
		}
		Err(err) => {
			error!("Could not create a new TcpListener");
			NetOpResult::Failure(err)
		}
	}
}

fn new_tcp_stream<T: ToSocketAddrs>(inetaddr: &T) -> NetOpResult<TcpStream> {
	match TcpStream::connect(inetaddr) {
		Ok(stream) => {
			info!("Created a new TcpStream!");
			NetOpResult::Success(stream)
		}
		Err(err) => {
			error!("Could not create a new TcpStream");
			NetOpResult::Failure(err)
		}
	}
}

fn new_udp_socket<T: ToSocketAddrs>(inetaddr: &T) -> NetOpResult<UdpSocket> {
	match UdpSocket::bind(inetaddr) {
		Ok(socket) => {
			info!("Created a new UdpSocket!");
			NetOpResult::Success(socket)
		}
		Err(err) => {
			error!("Could not create a new UdpSocket");
			NetOpResult::Failure(err)
		}
	}
}
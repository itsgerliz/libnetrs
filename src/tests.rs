#[cfg(test)]

use crate::netops::*;

#[test]
fn tcp_listener_invalid_addr() {
	let address = ("122.0.0.1", 25565);
	let listener = new_tcp_listener(&address);
	assert!(listener.is_failure())
}

#[test]
fn tcp_listener_valid_addr() {
	let address = ("127.0.0.1", 25565);
	let listener = new_tcp_listener(&address);
	assert!(listener.is_success())
}
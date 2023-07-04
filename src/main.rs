mod dhcp;

use std::env;
use std::net::UdpSocket;
use std::sync::Arc;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let server_socket = UdpSocket::bind("0.0.0.0:67")
        .expect("failed to bind socket");
    server_socket.set_broadcast(true).unwrap();

    // ヒープ上にDhcpServer構造体を確保し、複数のスレッドから共有するためArcを利用
    let dhcp_server = Arc::new(
        todo!()
    );

    
}

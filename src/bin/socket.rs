use std::net::ToSocketAddrs;

fn main() {
    let a = "google.com:443";
    let b = "baidu.com:443";
    println!("{:?}", a.to_socket_addrs());
    println!("{:?}", b.to_socket_addrs());
}

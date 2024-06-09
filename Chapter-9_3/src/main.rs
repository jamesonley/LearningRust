use std::net::IpAddr;


fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP Address should be valud");
}

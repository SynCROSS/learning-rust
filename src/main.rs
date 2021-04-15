fn main() {
    enum IP {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IP,
        address: String,
    }

    let home = IpAddr {
        kind: IP::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IP::V6,
        address: String::from("::1"),
    };

    println!("localhost: {:?}", &home.address);
    println!("loopback: {:?}", &loopback.address);
}

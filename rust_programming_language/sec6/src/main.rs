enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn show_ip_addr(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a1, a2, a3, a4) => {
            println!("This is an IpV4 address: {a1}, {a2}, {a3}, {a4}\n");
        }
        IpAddr::V6(addr) => {
            println!("This is an IpV6 address: {addr}\n");
        }
    }

}

fn get_possible_ip_addr(ip: Option<&IpAddr>) {
    match ip {
        Some(t) => {
            match t {
                IpAddr::V4(a1, a2, a3, a4) => {
                    println!("This is an IpV4 address: {a1}, {a2}, {a3}, {a4}\n");
                }
                IpAddr::V6(s) => {
                    println!("This is an IpV6 address: {s}\n");
                }
            };
        }
        None => {}
    }
}

fn main() {
    let v4 = IpAddr::V4(1, 2, 3, 4);
    let v6 = IpAddr::V6(String::from("127.0.0.1"));

    show_ip_addr(&v4);
    show_ip_addr(&v6);

    get_possible_ip_addr(Some(&v4));

    let v6 = IpAddr::V6(String::from("192.168.0.1"));
    if let IpAddr::V6(s) = v6 {
        println!("This is an IpV6, address = {s}\n");
    } else {
        println!("This is an IpV4\n");
    }

}

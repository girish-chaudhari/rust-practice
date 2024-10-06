
#[derive(Debug, Clone)]
enum IpAddrKind {
    V4(String),
    V6(String),
    
}

#[derive(Debug)]
struct  IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.1.1.3"));
    let six = IpAddrKind::V6(String::from("::1"));

    let home = IpAddr {
        kind: four.clone(),
        address: String::from("127.1.1.3"),
    };

    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };

    println!("home is {:#?}", home);
    println!("loopback is {:#?}", loopback);

    println!("four is {:#?}", four);

}

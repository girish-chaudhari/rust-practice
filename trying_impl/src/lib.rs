enum IpAddrKind {
    V4,
    V6,
}

struct IPAddr {
    // kind: IpAddrKind,
    // address: String,
    V4(String),
    V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// let home = IPAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("192:168:1:1"),
//     };

// let loopback = IpAddr {
//        kind: IpAddrKind::V6,
//        address: String::from("::1"),
//    };
//
const home = IpAddr::V4(String::from("127.0.0.1"));

const loopback = IpAddr::V6(String::from("::1"));

pub impl HelloWorld {
    fn say_hello() {
        println!("Hello, world from HelloWorld struct!");
    }
}


mod fn message_func() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

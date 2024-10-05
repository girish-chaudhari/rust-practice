// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IPAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// static HOME: IPAddr = IPAddr {
//     kind: IpAddrKind::V4,
//     address: "127.0.0.1".to_string(),
// };

// static LOOPBACK: IPAddr = IPAddr {
//     kind: IpAddrKind::V6,
//     address: "::1".to_string(),
// };

// pub struct HelloWorld;

// impl HelloWorld {
//     fn say_hello() {
//         println!("Hello, world from HelloWorld struct!");
//     }
// }

mod message_func {
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

    pub fn call_message() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }
}
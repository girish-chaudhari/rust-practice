#[derive(Debug)]
struct QuitMessage; // unit struct
#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct WriteMessage(String); // tuple struct
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug, Clone)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let q = QuitMessage;
    let m = MoveMessage { x: 1, y: 2 };
    let mut w = WriteMessage(String::from("Hello, world!"));
    let c = ChangeColorMessage(0, 160, 255);

    // let coin = if let Coin::Quarter(state) = Coin::Quarter(UsState::Alabama) {
    //     println!("State quarter from {:?}!", state);
    //     Coin::Quarter(state)
    // } else {
    //     Coin::Penny // Default to Penny if not a Quarter
    // };
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Nickel;
    // println!("coin value is {}", value_in_cents(coin));
    println!("coin value is {:?}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    w.0 = String::from("Hello, Rust!");

    println!("this is a unit struct: {:?}", q);
    println!("this is a tuple struct: {:?}", m);
    println!("this is a tuple struct: {:?}", w);
    println!("this is a tuple struct: {:?}", c);
    println!("Move message fields: {}, {}", m.x, m.y);
    println!("ChangeColorMessage fields: {}, {}, {}", c.0, c.1, c.2);

    // let's try something out that is not in the book
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);

    // let's try dice roll 
    let dice_roll = 7;
    match dice_roll {
        1 => println!("Rolled a 1!"),
        2 => println!("Rolled a 2!"),
        3 => println!("Rolled a 3!"),
        4 => println!("Rolled a 4!"),
        5 => println!("Rolled a 5!"),
        6 => println!("Rolled a 6!"),
        _ => println!("That's not a valid dice roll!"),
    }


    let mut count = 0;
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("Oops Coin does not match for this...");
        println!("The count of the number is {}", count)
    }

    let fancy_hat = add_fancy_hat(coin.clone());
    println!("The coin with fancy hat is {:?}", fancy_hat);

    let plain_hat = remove_fancy_hat(coin.clone());
    println!("The coin with plain hat is {:?}", plain_hat);
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

    // let's try out the if let syntax for Option<T>
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("Not three"),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }

}

fn add_fancy_hat(coin: Coin) -> Coin {
    match coin {
        Coin::Penny => Coin::Penny,
        Coin::Nickel => Coin::Nickel,
        Coin::Dime => Coin::Dime,
        Coin::Quarter(state) => Coin::Quarter(state),
    }
}
fn remove_fancy_hat(coin: Coin) -> Coin {
    match coin {
        Coin::Penny => Coin::Penny,
        Coin::Nickel => Coin::Nickel,
        Coin::Dime => Coin::Dime,
        Coin::Quarter(state) => Coin::Quarter(state),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

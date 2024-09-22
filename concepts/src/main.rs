mod conditional;
mod ch_strings;
mod ownership;
use ownership::ownership::*;

fn main() {
    let number = 6;
    {
        let condition = true;

        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // test_condition();
    counter();
    counting_up();

    conditional::test_condition();
    conditional::lift_off();
    conditional::using_for_loop();
    conditional::reverse_iteration();
    // conditional::collection_through_loop();

    // lets talk about string and ownership
    ch_strings::string_slice();
    // ownership::data_string_ownership(); use all the functions from ownership module
    data_string_ownership()
    // ownership::data_string_ownership();
}

fn counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

fn counting_up() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    println!("LIFTOFF!!!");
}

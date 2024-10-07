use crate::garden::vegetables::Asparagus;

// pub mod Asparagus {
//     pub mod Vegetable {
//         pub fn cook() {
//             println!("Cooking asparagus!");
//         }
//     }
// }

// pub mod garden;


use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
pub fn test_condition() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}");
}

pub fn lift_off() {
  let mut counter = 0;

  while counter != 0 {
    counter -= 1;
  }

  println!("lift_top: counter = {counter}");
}

// This function demonstrates how to iterate through a collection using a loop.
#[allow(dead_code)]
pub fn collection_through_loop() {
  let a: [i32; 5] = [10, 20, 30, 40, 50];
  let mut index = 0;

  // while index < 5 {
  //   println!("the value is: {}", a[index]);
  //   index += 1;
  // }

  while index < a.len() {
    println!("the value is: {}", a[index]);
    index += 1;
  }
}

pub fn using_for_loop() {
  let a: [i32; 5] = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {} using for loop iteration", element);
  }
}
// #[allow(dead_code)]
pub fn reverse_iteration() {
  let a: [i32; 5] = [10, 20, 30, 40, 50];

  for number in (1..a.len()).rev() {
    println!("the value is: {} using reverse iteration", a[number]);
  }
}
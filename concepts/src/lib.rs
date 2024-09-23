pub mod conditional;
pub mod mods;

#[allow(dead_code)]
pub  mod ownership {
 pub fn test_db() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {s1}, s2 = {s2}");


}
pub fn try_ownership_with_int() {
  let x = 5;
  let y = x;

  println!("x = {x}, y = {y}");
  
}
}

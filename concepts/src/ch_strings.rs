pub fn string_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut_string = String::from("This is mutable string");
    let first_word = mut_string.split_whitespace().next().unwrap();

  // let arr_of_strings = mut_string.split_whitespace().collect::<Vec<&str>>();  
  // println!("arr_of_strings: {arr_of_strings}");

    println!("first_word: {first_word}");

    println!("hello: {hello}");
    println!("world: {world}");
}
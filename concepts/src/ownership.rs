// export all the ownership related concepts
pub mod ownership {
    // pub fn string_slice() {
    //     let s = String::from("hello world");

    //     let hello = &s[0..5];
    //     let world = &s[6..11];

    //     println!("hello: {hello}, world: {world}");
    // }
    pub fn data_string_ownership() {
      let s1 = String::from("hello");
      let s2 = s1.clone();
  
      println!("s1 = {s1}, s2 = {s2}");
    }
}
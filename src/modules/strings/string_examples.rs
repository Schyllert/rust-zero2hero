pub fn string_example1() {
  let s1: &str = "Tjena tjena 😛";

  let s3: String = "Tjena tjena 😛".to_string();
  let s4: String = "Tjena tjena 😛".to_owned();
  let s5: &str = &s4[0..1];
}

pub fn manipulating_strings() {
  let mut s2: String = String::from("Tjena tjena 😛");

  println!("{}", s2);
  s2.push_str(" walla");
  println!("{}", s2);
  s2.replace_range(.., "replace_with");
  // Replaced the entire string with "replace_with"
  println!("{}", s2);
}

pub fn concatinating_strings() {
  let s1 = String::from("Hello ");
  println!("{}", s1);
  let s2 = String::from("world!");
  let s3 = s1 + &s2;
  println!("{}", s3);

  // This will no longer work - "value barrowed here after move"
  // A variable was used after its contents have been moved elsewhere
  //println!("{}", s1);
}

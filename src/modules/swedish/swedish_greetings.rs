pub fn greeting() {
  println!("Hej");
}

// We will not be able to use this function in our tests due to it being private
fn private_greeting() {
  println!("Privat hej!")
}

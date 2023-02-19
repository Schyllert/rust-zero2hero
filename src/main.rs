// Is this information to the extension or the compiler?
#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod modules;
use modules::{english, swedish, tests};

// We have to add this in the cargo.toml
// rand = "0.8"
use rand::Rng;

fn main() {
  println!("Hello, world!");
  swedish::swedish_greetings::greeting();
  english::english_greetings::greeting();

  statements();
}

pub fn statements() {
  // A let statement introduces a new set of variables, given by a pattern.
  let x: i32 = 5;

  // When no type annotation is given, the compiler will infer the type, or signal an error if insufficient type information is available for definite inference.
  let y = 5.0;

  // If statement
  if 1 == 2 + 4 {
  } else {
  }

  // Switch statement
  let mut x: i32 = give_number();
  match x {
    1 => println!("You got number 1"),
    2 => println!("You got number 2"),
    _ => println!("Default"),
  }

  // For statement
  for x in 0..3 {
    println!("value of iterator is: {}", x);
  }

  // Infinite loop
  loop {
    x += 1;
    if x > 10 {
      println!("We are out of here");
      break;
    }
  }
}

// Rust has no classes xd
// Modules...
// Always have to have a main
// Everything is private by default use keyword pub to make it public

/*
1) cargo run
2) cargo clippy

 */

fn give_number() -> i32 {
  let mut rng = rand::thread_rng();
  return rng.gen_range(0..2);
}

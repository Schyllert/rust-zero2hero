#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod modules;
use modules::{english, swedish, tests};

fn main() {
  println!("Hello, world!");
  swedish::swedish_greetings::greeting();
  english::english_greetings::greeting();

  statements();
}

pub fn statements() {
  // Let statements

  // A let statement introduces a new set of variables, given by a pattern.

  let x: i32 = 5;

  // When no type annotation is given, the compiler will infer the type, or signal an error if insufficient type information is available for definite inference.
  let y = 5.0;
}

// Rust has no classes xd
// Modules...
// Always have to have a main
// Everything is private by default use keyword pub to make it public

/*
1) cargo run
2) cargo clippy

 */

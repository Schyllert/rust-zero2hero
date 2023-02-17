#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod modules;
use modules::{swedish_greetings, english_greetings};


fn main() {
    println!("Hello, world!");
    swedish_greetings::swedish_greetings::greeting();
}



// Rust has no classes xd
// Modules...
// Always have to have a main
// Everything is private by default use keyword pub to make it public


/*
1) cargo run
2) cargo clippy 

 */








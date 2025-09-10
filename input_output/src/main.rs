use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    println!("Please enter some input: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input.trim());
    let path = "output.txt";
    let mut file = File::create(path).expect("Could not create file");
    file.write_all(input.as_bytes()).expect("Could not write to file");
    println!("Input written to {}", path);

    
}

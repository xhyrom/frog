use std::{env};

pub fn handle() -> () {
    #[cfg(debug_assertions)]
    println!("frog {} (debug)", env!("CARGO_PKG_VERSION"));
    #[cfg(not(debug_assertions))]
    println!("frog {} (release)", env!("CARGO_PKG_VERSION"));
}
#[macro_use]
extern crate bitflags;

mod binary;

#[cfg(feature = "debug")]
macro_rules! debug {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($( $args:expr ),*) => {};
}

#[macro_use]
pub mod error;
pub mod class;
pub mod io;
pub mod vm;

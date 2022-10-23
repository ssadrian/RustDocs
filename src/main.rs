//! Task

#![warn(missing_docs)]
#![allow(unused)]

use crate::generics::{Candy, Comestible};
use crate::pastila::Pastila;

mod pastila;
mod generics;

/// Entry point
fn main() {
    let mut candy: Candy = Candy::new(String::from("Red Vine"));
    let mut pastila: Pastila = Pastila::new(String::from("Apple Pastila"));

    candy.consume();
    pastila.consume();

    let switch_successful: bool = candy.try_giving_candy_to(Some({ "John" }));
    pastila.decorate_with(candy);
}

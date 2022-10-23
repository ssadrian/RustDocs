use std::fmt::{Debug, Display};

pub use crate::generics::candy::Candy;
pub use crate::generics::comestible::Comestible;
use crate::Pastila;

pub mod comestible;
pub mod candy_state;
pub mod candy;

impl Comestible for Candy {
    fn consume(&mut self) {
        println!("{} consumed", self.name);
        self.is_consumed = true;
    }

    fn was_consumed(&self) -> bool {
        return self.is_consumed;
    }

    fn name(&mut self) -> &str {
        &mut self.name
    }
}

impl Comestible for Pastila {
    fn consume(&mut self) {
        println!("{} consumed", self.name);
        self.is_consumed = true;
    }

    fn was_consumed(&self) -> bool {
        self.is_consumed
    }

    fn name(&mut self) -> &str {
        &mut self.name
    }
}

use std::fmt::{Debug, Display};
use crate::generics::comestible::Comestible;

pub struct Candy {
    /// Track the consumed state of the candy
    pub is_consumed: bool,

    /// The name of the candy
    pub name: String,
}

impl Candy {
    /// Create a new named candy
    ///
    /// # Arguments
    /// * `name`: The name of the candy
    ///
    /// # Examples
    ///
    /// ```
    /// let candy: Candy = Candy::new("Honey Pumpkin Slices");
    /// ```
    pub fn new(name: String) -> Self {
        Self { is_consumed: false, name }
    }

    /// Switch the owner of the candy to someone
    ///
    /// # Arguments
    ///
    /// * `someone`: The person to receive the pastila
    ///
    /// # Panics
    /// When the receiving person does not exist or the pastila is already consumed
    ///
    /// # Examples
    ///
    /// ```run
    /// let candy: Candy = Candy::new("Honey Pumpkin Slices");
    /// candy.give_candy_to(Some({ "Jason Voorhees" }));
    /// ```
    #[deprecated]
    pub fn give_candy_to<T: Display>(&mut self, someone: Option<T>) {
        if self.was_consumed() {
            panic!("The {} is already consumed", &self.name())
        }

        match &someone {
            Some(person) => {
                println!("Candy gave to {}", person);
            }
            None => panic!("The other consumer was not found")
        }
    }

    /// Switch the owner of the candy to someone
    ///
    /// # Arguments
    ///
    /// * `someone`: The person to receive the candy
    ///
    /// return (bool): True when switch was successful; otherwise False
    ///
    /// # Examples
    ///
    /// ```
    /// let candy: Candy = Candy::new("Honey Pumpkin Slices");
    /// let success: bool = candy.try_giving_candy_to(Some({ "Jason Voorhees" }));
    /// // handle the result
    /// ```
    pub fn try_giving_candy_to<T: Debug>(&self, someone: Option<T>) -> bool {
        if self.was_consumed() {
            return false;
        }

        return match someone {
            Some(person) => {
                println!("Candy gave to {:#?}", <T as TryInto<T>>::try_into(person));
                true
            }
            None => false
        };
    }
}
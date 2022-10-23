use std::fmt::Display;

use super::Comestible;

/// A traditional Russian fruit confectionery.
/// It's a small square of pressed fruit paste and light, airy puffs with a delicate apple flavor
pub struct Pastila {
    /// Track the consumed state of the pastila
    pub is_consumed: bool,

    /// Name of the pastila
    pub name: String,
}

impl Pastila {
    /// Create a new named pastila
    ///
    /// # Arguments
    /// * `name`: The name of the pastila
    ///
    /// # Examples
    ///
    /// ```
    /// let pastila: Pastila = Pastila::new("Apple Pastila");
    /// ```
    pub fn new(name: String) -> Self {
        Self { is_consumed: false, name }
    }

    /// Get the flavour of the pastila
    pub fn flavour(&self) -> String {
        String::from(&self.name)
    }

    /// Add the `decoration` to the pastila
    ///
    /// The `decoration` can be anything that implements the [`Comestible`] trait
    ///
    /// # Arguments
    ///
    /// * `decoration`: The candy based decoration to decorate with
    ///
    /// # Examples
    ///
    /// ```
    /// let applePastila: Pastila = Pastila::new("Apple Pastila");
    /// let pearPastila: Pastila = Pastila::new("Pear Pastila");
    /// applePastila.decorate_with(pearPastila);
    /// ```
    pub fn decorate_with<T: Comestible>(&mut self, mut decoration: T) {
        println!("{} decorated with {}", self.name(), decoration.name());
    }
}
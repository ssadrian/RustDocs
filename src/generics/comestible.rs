/// Representation of a comestible
pub trait Comestible {
    /// Consume the comestible
    fn consume(&mut self);

    /// Determine if the comestible was consumed
    ///
    /// returns(bool) - True if the comestible is consumed; otherwise False
    fn was_consumed(&self) -> bool;

    /// Set and get the name of the comestible
    fn name(&mut self) -> &str;
}
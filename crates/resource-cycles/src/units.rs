//! Units of measurement for various resources.

use crate::Reflective;

/// `Units` of measurement for a resource, such as "kg", "m", "s", etc.
pub trait Units: Reflective {}

/// `Time` represents a unit of time, such as "s", "hr", "day", etc.
pub trait Time: Units {
    /// Returns the number of seconds in one unit of this time.
    fn seconds_per_unit() -> f64;
}

/// A `Second` is a unit of time equal to one second.
#[derive(Reflective)]
#[extra_names(extra_names = ["sec", "s"])]
pub struct Second;

impl Units for Second {}

impl Time for Second {
    fn seconds_per_unit() -> f64 {
        1.0
    }
}

/// A `Kilogram` is a unit of mass equal to one kilogram.
#[derive(Reflective)]
#[extra_names(extra_names = ["kg"])]
pub struct Kilogram;

impl Units for Kilogram {}

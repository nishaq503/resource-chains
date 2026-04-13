//! Units used to measure time in the game "Oxygen Not Included".

use resource_chains::{
    Reflective,
    units::{Time, Units},
};

/// In "Oxygen Not Included", a "[cycle](https://oxygennotincluded.wiki.gg/wiki/Cycles)" is 600 seconds.
#[derive(Reflective)]
pub struct Cycle;

impl Units for Cycle {}

impl Time for Cycle {
    fn seconds_per_unit() -> f64 {
        600.0
    }
}

/// A `Second` is a unit of time equal to one second.
#[derive(Reflective, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[reflective(extra_names = ["sec", "s"])]
pub struct Second;

impl Units for Second {}

impl Time for Second {
    fn seconds_per_unit() -> f64 {
        1.0
    }
}

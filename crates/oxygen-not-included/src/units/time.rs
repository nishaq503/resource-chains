//! Units used to measure time in the game "Oxygen Not Included".

use resource_cycles::{
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

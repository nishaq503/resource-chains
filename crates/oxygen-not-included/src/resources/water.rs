//! The various kinds of water in the game. These include pure [water](https://oxygennotincluded.wiki.gg/wiki/Water), [polluted water](https://oxygennotincluded.wiki.gg/wiki/Polluted_Water), [salt water](https://oxygennotincluded.wiki.gg/wiki/Salt_Water), and [brine](https://oxygennotincluded.wiki.gg/wiki/Brine).

use resource_cycles::{Reflective, Resource, units::Kilogram};

/// Pure [Water](https://oxygennotincluded.wiki.gg/wiki/Water) is measured in kilograms (kg).
#[derive(Reflective)]
pub struct Water;

impl Resource for Water {
    type Units = Kilogram;

    fn known_consumers(&self) -> Vec<&str> {
        vec!["ovagro-node"]
    }

    fn known_producers(&self) -> Vec<&str> {
        vec!["water-sieve"]
    }
}

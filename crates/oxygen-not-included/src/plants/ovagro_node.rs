//! An [Ovagro Node](https://oxygennotincluded.wiki.gg/wiki/Ovagro_Node).

use resource_cycles::{Process, Reflective};

/// An [`OvagroNode`] is a plant in the game.
#[derive(Reflective)]
pub struct OvagroNode;

impl Process for OvagroNode {
    type TimePerIteration = crate::units::Cycle;

    fn consumed_resources(&self) -> Vec<(&str, f64)> {
        vec![("water", 90.0)]
    }

    fn produced_resources(&self) -> Vec<(&str, f64)> {
        todo!()
    }
}

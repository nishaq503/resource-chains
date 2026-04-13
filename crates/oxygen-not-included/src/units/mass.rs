//! Units to measure mass in the game.

use resource_chains::prelude::*;

/// A `Kilogram` is a unit of mass equal to one kilogram.
#[derive(Reflective, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[reflective(extra_names = ["kg"])]
pub struct Kilogram;

impl Units for Kilogram {}

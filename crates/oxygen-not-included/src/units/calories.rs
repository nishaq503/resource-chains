//! Calories measure food in the game.

use resource_cycles::{Reflective, units::Units};

/// In "Oxygen Not Included", a "kcal" (kilocalorie) is the unit of energy provided by food.
#[derive(Reflective)]
pub struct Kcal;

impl Units for Kcal {}

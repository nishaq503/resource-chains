//! A `Process` represents an edge in the graph, connecting two or more resources.

use crate::{Reflective, units::Time};

/// A `Process` represents an edge in the graph, connecting two or more resources.
pub trait Process: Reflective {
    /// The time taken for one iteration of this process.
    type TimePerIteration: Time;

    /// The names and quantities of all resources consumed by this process.
    fn consumed_resources(&self) -> Vec<(&str, f64)>;

    /// The names and quantities of all resources produced by this process.
    fn produced_resources(&self) -> Vec<(&str, f64)>;
}

/// A `Device` represents an entity that can perform processes, such as a machine, a person, etc.
pub trait Device: Reflective {}

/// An `ActualizedProcess` is a process that must be performed by a specific `Device`.
pub trait ActualizedProcess: Process {
    /// The device that must perform this process.
    type Device: Device;
}

/// A `Person` is a device that can perform processes.
#[derive(Reflective)]
pub struct Person;

impl Device for Person {}

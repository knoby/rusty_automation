#![allow(unused)]
pub mod hal;

pub mod prelude {
    use crate::{DigitalInput, DigitalOutput};
}

pub trait DigitalInput {
    fn get_state(&self) -> bool;
}

pub trait DigitalOutput {
    fn set_output(&mut self, value: bool);
    fn set_true(&mut self);
    fn set_false(&mut self);
}

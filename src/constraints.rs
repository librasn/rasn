use core::ops::Range;

pub enum Size {
    Fixed(usize),
    Range(Range<usize>),
}

pub trait Constraints {
    const SIZE: Option<Size> = None;
}

pub struct Unconstrained;

impl Constraints for Unconstrained {}

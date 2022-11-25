use bui::{line::LineRaw, freeform_2dcapsule::Freeform2DCapsule};

pub trait Construct<C> {
    fn construct(&self) -> C;
}

pub trait StandardConstructTarget {
    const EMPTY: Self;
    fn append(&mut self, value: Self);
    fn append_into(self, value: Self) -> Self;
}

#[derive(Debug, Clone)]
pub struct LineTarget (pub Vec<LineRaw>);

impl StandardConstructTarget for LineTarget {
    const EMPTY: Self = LineTarget(vec![]);

    fn append(&mut self, mut value: Self) {
        self.0.append(&mut value.0)
    }

    fn append_into(mut self, mut value: Self) -> Self {
        self.0.append(&mut value.0);
        self
    }
}

impl Default for LineTarget {
    fn default() -> Self {
        Self::EMPTY
    }
}

#[derive(Debug, Clone)]
pub struct Freeform2DCapsuleTarget (pub Vec<Freeform2DCapsule>);

impl StandardConstructTarget for Freeform2DCapsuleTarget {
    const EMPTY: Self = Freeform2DCapsuleTarget(vec![]);

    fn append(&mut self, mut value: Self) {
        self.0.append(&mut value.0);
    }

    fn append_into(mut self, mut value: Self) -> Self {
        self.0.append(&mut value.0);
        self
    }
}
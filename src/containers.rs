use std::marker::PhantomData;

use bui::rect::{SizeAndCenter, Points};

use crate::{construct::Construct, signal::SignalReciever};

pub struct FillContainer<T> {
    child: T,
    fill_target: SizeAndCenter,
}

impl<T> FillContainer<T> {
    pub fn new(child: T, fill_target: SizeAndCenter) -> Self {
        Self {
            child,
            fill_target,
        }
    }
}

impl<C, T: Construct<C>> Construct<C> for FillContainer<T> {
    fn construct(&self) -> C {
        self.child.construct()
    }
}

impl<T> GetHeight for FillContainer<T> {
    fn get_height(&self) -> f32 {
        self.fill_target.sy*2.0
    }
}

impl<T> SetCenterTopPosition for FillContainer<T> {
    fn set_center_top_position(&mut self, x: f32, y: f32) {
        self.fill_target.cx = x;
        self.fill_target.cy = y-self.fill_target.sy;
    }
}

impl<T: Fill> Init for FillContainer<T> {
    fn init(&mut self) {
        self.child.fill(self.fill_target);
    }
}

impl<R, T: SignalReciever<S, R>, S> SignalReciever<S, R> for FillContainer<T> {
    fn take_signal(&mut self, signal: &mut S) -> R {
        self.child.take_signal(signal)
    }
}

pub trait Fill {
    fn fill(&mut self, fill_target: SizeAndCenter);
}

pub trait Init {
    fn init(&mut self);
}

pub trait GetHeight {
    fn get_height(&self) -> f32;
}

pub trait GetCenterPosition {
    fn get_center_position(&self) -> (f32, f32);
}

pub trait SetCenterTopPosition {
    fn set_center_top_position(&mut self, x: f32, y: f32);
}

pub trait GetPointBounds {
    fn get_point_bounds(&self) -> Points;
}
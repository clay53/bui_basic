use bui::rect::SizeAndCenter;

use crate::{construct::{ Construct, StandardConstructTarget }, containers::{Init, GetHeight, SetCenterTopPosition, Fill}, signal::SignalReciever};

#[derive(Debug, Clone)]
pub struct VecColumn<T> {
    children: Vec<T>,
    cx: f32,
    top_y: f32,
}

impl<T> VecColumn<T> {
    pub fn new() -> Self {
        VecColumn {
            children: Vec::new(),
            cx: 0.0,
            top_y: 1.0,
        }
    }

    pub fn push(mut self, value: T) -> Self {
        self.children.push(value);
        self
    }

    pub fn set_top_y(&mut self, top_y: f32) {
        self.top_y = top_y;
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<T> {
        &mut self.children
    }

    pub fn reposition_children(&mut self)
    where
        T: GetHeight+SetCenterTopPosition
    {
        let mut y = self.top_y;
        for child in &mut self.children {
            child.set_center_top_position(self.cx, y);
            y -= child.get_height();
        }
    }
}

impl<T> From<Vec<T>> for VecColumn<T> {
    fn from(children: Vec<T>) -> Self {
        VecColumn {
            children,
            cx: 0.0,
            top_y: 0.0
        }
    }
}

impl<C: StandardConstructTarget, T: Construct<C>> Construct<C> for VecColumn<T> {
    fn construct(&self) -> C {
        if self.children.len() == 0 {
            C::EMPTY
        } else {
            let mut construct_target = self.children[0].construct();
            for i in 1..self.children.len() {
                construct_target.append(self.children[i].construct());
            }
            construct_target
        }
    }
}

impl<T: GetHeight+SetCenterTopPosition+Init> Init for VecColumn<T> {
    fn init(&mut self) {
        let mut y = self.top_y;
        for child in &mut self.children {
            child.set_center_top_position(self.cx, y);
            y -= child.get_height();
            child.init();
        }
    }
}

impl<T: SignalReciever<S, R>, S, R> SignalReciever<S, Vec<R>> for VecColumn<T> {
    fn take_signal(&mut self, signal: &mut S) -> Vec<R> {
        let mut responses = Vec::with_capacity(self.children.len());
        for child in &mut self.children {
            responses.push(child.take_signal(signal));
        }
        responses
    }
}
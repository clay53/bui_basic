use std::{ops::IndexMut, marker::PhantomData};

use crate::{containers::{Init, FillWidth, TranslateX, TranslateY}, construct::Construct, signal::SignalReciever};

pub struct SwitchContainer<A: Copy, T: ?Sized, D: IndexMut<A, Output=T>> {
    active: A,
    data: D,
    active_data: PhantomData<T>,
}

impl<A: Copy, T: ?Sized, D: IndexMut<A, Output=T>> SwitchContainer<A, T, D> {
    pub fn new(active: A, data: D) -> Self {
        Self {
            active,
            data,
            active_data: PhantomData,
        }
    }
}

impl<A: Copy, T: ?Sized+Init, D: IndexMut<A, Output=T>> Init for SwitchContainer<A, T, D> {
    fn init(&mut self) {
        self.data[self.active].init()
    }
}

impl<A: Copy, T: ?Sized+FillWidth, D: IndexMut<A, Output=T>> FillWidth for SwitchContainer<A, T, D> {
    fn fill_width(&mut self, sx: f32, cx: f32, ty: f32) -> f32 {
        self.data[self.active].fill_width(sx, cx, ty)
    }
}

impl<A: Copy, T: ?Sized+TranslateX, D: IndexMut<A, Output=T>> TranslateX for SwitchContainer<A, T, D> {
    fn translate_x(&mut self, dx: f32) {
        self.data[self.active].translate_x(dx)
    }
}

impl<A: Copy, T: ?Sized+TranslateY, D: IndexMut<A, Output=T>> TranslateY for SwitchContainer<A, T, D> {
    fn translate_y(&mut self, dy: f32) {
        self.data[self.active].translate_y(dy)
    }
}

impl<C, A: Copy, T: ?Sized+Construct<C>, D: IndexMut<A, Output=T>> Construct<C> for SwitchContainer<A, T, D> {
    fn construct(&self) -> C {
        self.data[self.active].construct()
    }
}

impl<S, O, A: Copy, T: ?Sized+SignalReciever<S, O>, D: IndexMut<A, Output=T>> SignalReciever<S, O> for SwitchContainer<A, T, D> {
    fn take_signal(&mut self, signal: &mut S) -> O {
        self.data[self.active].take_signal(signal)
    }
}
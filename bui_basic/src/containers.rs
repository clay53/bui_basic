use std::marker::PhantomData;

use bui::rect::{SizeAndCenter, Points};

use crate::{construct::{Construct, StandardConstructTarget}, signal::{SignalReciever, ShortCircuitingCallback}};

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

pub struct HeightContainer<T> {
    child: T,
    sy: f32,
}

impl<T> HeightContainer<T> {
    pub fn new(child: T, sy: f32) -> Self {
        Self {
            child,
            sy,
        }
    }
}

impl<T: Init> Init for HeightContainer<T> {
    fn init(&mut self) {
        self.child.init();
    }
}

impl<T: Fill> FillWidth for HeightContainer<T> {
    fn fill_width(&mut self, sx: f32, cx: f32, ty: f32) -> f32 {
        self.child.fill(SizeAndCenter {
            sx,
            sy: self.sy,
            cx,
            cy: ty-self.sy,
        });
        ty-self.sy*2.0
    }
}

impl<C, T: Construct<C>> Construct<C> for HeightContainer<T> {
    fn construct(&self) -> C {
        self.child.construct()
    }
}

impl<T: TranslateX> TranslateX for HeightContainer<T> {
    fn translate_x(&mut self, dx: f32) {
        self.child.translate_x(dx);
    }
}

impl<T: TranslateY> TranslateY for HeightContainer<T> {
    fn translate_y(&mut self, dy: f32) {
        self.child.translate_y(dy);
    }
}

impl<R, T: SignalReciever<S, R>, S> SignalReciever<S, R> for HeightContainer<T> {
    fn take_signal(&mut self, signal: &mut S) -> R {
        self.child.take_signal(signal)
    }
}

pub struct HSplitContainer<L, R> {
    left: L,
    split: f32,
    right: R,
}

impl<L, R> HSplitContainer<L, R> {
    pub fn new(left: L, split: f32, right: R) -> Self {
        Self {
            left,
            split,
            right,
        }
    }
}

impl<L: Init, R: Init> Init for HSplitContainer<L, R> {
    fn init(&mut self) {
        self.left.init();
        self.right.init();
    }
}

impl<L: Fill, R: Fill> Fill for HSplitContainer<L, R> {
    fn fill(&mut self, fill_target: SizeAndCenter) {
        let (left_target, right_target) = fill_target.split_h(self.split);
        self.left.fill(left_target);
        self.right.fill(right_target);
    }
}

impl<L: TranslateX, R: TranslateX> TranslateX for HSplitContainer<L, R> {
    fn translate_x(&mut self, dx: f32) {
        self.left.translate_x(dx);
        self.right.translate_x(dx);
    }
}

impl<L: TranslateY, R: TranslateY> TranslateY for HSplitContainer<L, R> {
    fn translate_y(&mut self, dy: f32) {
        self.left.translate_y(dy);
        self.right.translate_y(dy);
    }
}

impl<C: StandardConstructTarget, L: Construct<C>, R: Construct<C>> Construct<C> for HSplitContainer<L, R> {
    fn construct(&self) -> C {
        self.left.construct().append_into(self.right.construct())
    }
}

impl<O, P, L: SignalReciever<S, O>, R: SignalReciever<S, P>, S> SignalReciever<S, (O, P)> for HSplitContainer<L, R> {
    fn take_signal(&mut self, signal: &mut S) -> (O, P) {
        (self.left.take_signal(signal), self.right.take_signal(signal))
    }
}

impl<O: ShortCircuitingCallback, L: SignalReciever<S, O>, R: SignalReciever<S, O>, S> SignalReciever<S, O> for HSplitContainer<L, R> {
    fn take_signal(&mut self, signal: &mut S) -> O {
        self.left.take_signal(signal).or_into(self.right.take_signal(signal))
    }
}

pub struct NoContainer {}

impl Fill for NoContainer {fn fill(&mut self, _fill_target: SizeAndCenter) {}}
impl FillWidth for NoContainer {fn fill_width(&mut self, _sx: f32, _cx: f32, ty: f32) -> f32 {ty}}
impl Init for NoContainer {fn init(&mut self) {}}
impl GetHeight for NoContainer {fn get_height(&self) -> f32 {0.0}}
impl GetCenterPosition for NoContainer {fn get_center_position(&self) -> (f32, f32) {(0.0, 0.0)}}
impl SetCenterTopPosition for NoContainer {fn set_center_top_position(&mut self, _cx: f32, _ty: f32) {}}
impl TranslateX for NoContainer {fn translate_x(&mut self, _dx: f32) {}}
impl TranslateY for NoContainer {fn translate_y(&mut self, _dy: f32) {}}
impl GetPointBounds for NoContainer {fn get_point_bounds(&self) -> Points {Points::ZERO}}
impl<T, R: Default> SignalReciever<T, R> for NoContainer {fn take_signal(&mut self, _signal: &mut T) -> R {R::default()}}
impl<C: Default> Construct<C> for NoContainer {fn construct(&self) -> C {C::default()}}

pub struct VStackContainer<T, B> {
    top: T,
    bottom: B,
}

impl<T, B> VStackContainer<T, B> {
    pub fn new(top: T, bottom: B) -> Self {
        Self {
            top,
            bottom,
        }
    }
}

impl<T: Init, B: Init> Init for VStackContainer<T, B> {
    fn init(&mut self) {
        self.top.init();
        self.bottom.init();
    }
}

impl<T: FillWidth, B: FillWidth> FillWidth for VStackContainer<T, B> {
    fn fill_width(&mut self, sx: f32, cx: f32, ty: f32) -> f32 {
        self.bottom.fill_width(sx, cx, self.top.fill_width(sx, cx, ty))
    }
}

impl<T: TranslateX, B: TranslateX> TranslateX for VStackContainer<T, B> {
    fn translate_x(&mut self, dx: f32) {
        self.top.translate_x(dx);
        self.bottom.translate_x(dx);
    }
}

impl<T: TranslateY, B: TranslateY> TranslateY for VStackContainer<T, B> {
    fn translate_y(&mut self, dy: f32) {
        self.top.translate_y(dy);
        self.bottom.translate_y(dy);
    }
}

impl<C: StandardConstructTarget, T: Construct<C>, B: Construct<C>> Construct<C> for VStackContainer<T, B> {
    fn construct(&self) -> C {
        self.top.construct().append_into(self.bottom.construct())
    }
}

impl<S, O, P, T: SignalReciever<S, O>, B: SignalReciever<S, P>> SignalReciever<S, (O, P)> for VStackContainer<T, B> {
    fn take_signal(&mut self, signal: &mut S) -> (O, P) {
        (self.top.take_signal(signal), self.bottom.take_signal(signal))
    }
}

impl<S, O: ShortCircuitingCallback, T: SignalReciever<S, O>, B: SignalReciever<S, O>> SignalReciever<S, O> for VStackContainer<T, B> {
    fn take_signal(&mut self, signal: &mut S) -> O {
        self.top.take_signal(signal).or_into(self.bottom.take_signal(signal))
    }
}

pub trait Fill {
    fn fill(&mut self, fill_target: SizeAndCenter);
}

pub trait FillWidth {
    fn fill_width(&mut self, sx: f32, cx: f32, ty: f32) -> f32;
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
    fn set_center_top_position(&mut self, cx: f32, ty: f32);
}

pub trait TranslateX {
    fn translate_x(&mut self, dx: f32);
}

pub trait TranslateY {
    fn translate_y(&mut self, dy: f32);
}

pub trait GetPointBounds {
    fn get_point_bounds(&self) -> Points;
}

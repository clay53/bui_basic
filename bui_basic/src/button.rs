use bui::rect::{SizeAndCenter, Points};

use crate::{construct::Construct, containers::{Fill, Init, GetHeight, TranslateY, TranslateX}, signal::{SignalReciever, ResizedSignal, CursorMovedSignal, MouseLeftDownSignal, MouseLeftUpSignal, CharacterInputSignal}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressState {
    None,
    Hovered,
    Pressed,
}

#[derive(Debug, Clone)]
pub struct Button<T> {
    child: T,
    press_state: PressState,
    fill_area: SizeAndCenter,
    points: Points,
}

impl<T> Button<T> {
    pub fn new(child: T, fill_area: SizeAndCenter) -> Self {
        Self {
            child,
            press_state: PressState::None,
            fill_area,
            points: fill_area.into(),
        }
    }
}

impl<C, T: Construct<C>> Construct<C> for Button<T> {
    fn construct(&self) -> C {
        self.child.construct()
    }
}

impl<T: Fill> Init for Button<T> {
    fn init(&mut self) {
        self.child.fill(self.fill_area);
    }
}

impl<T: Fill> Fill for Button<T> {
    fn fill(&mut self, fill_target: SizeAndCenter) {
        self.fill_area = fill_target;
        self.points = self.fill_area.into();
        self.child.fill(fill_target);
    }
}

impl <T: TranslateX> TranslateX for Button<T> {
    fn translate_x(&mut self, dx: f32) {
        self.fill_area.cx += dx;
        self.points.p1y += dx;
        self.points.p2y += dx;
        self.child.translate_x(dx);
    }
}

impl <T: TranslateY> TranslateY for Button<T> {
    fn translate_y(&mut self, dy: f32) {
        self.fill_area.cy += dy;
        self.points.p1y += dy;
        self.points.p2y += dy;
        self.child.translate_y(dy);
    }
}

impl<T> GetHeight for Button<T> {
    fn get_height(&self) -> f32 {
        self.fill_area.sy*2.0
    }
}

impl<T: SignalReciever<ResizedSignal, R>, R> SignalReciever<ResizedSignal, R> for Button<T> {
    fn take_signal(&mut self, signal: &mut ResizedSignal) -> R {
        self.child.take_signal(signal)
    }
}

impl<T> SignalReciever<CursorMovedSignal, PressStateCallback> for Button<T> {
    fn take_signal(&mut self, signal: &mut CursorMovedSignal) -> PressStateCallback {
        if self.points.contains(signal.norm_posx, signal.norm_posy) {
            match self.press_state {
                PressState::Hovered | PressState::Pressed => PressStateCallback::NoChange,
                PressState::None => {
                    self.press_state = PressState::Hovered;
                    PressStateCallback::Update(self.press_state)
                }
            }
        } else {
            match self.press_state {
                PressState::Hovered | PressState::Pressed => {
                    self.press_state = PressState::None;
                    PressStateCallback::Update(self.press_state)
                },
                PressState::None => PressStateCallback::NoChange
            }
        }
    }
}

impl<T, R: Default> SignalReciever<CursorMovedSignal, R> for Button<T> {
    fn take_signal(&mut self, signal: &mut CursorMovedSignal) -> R {
        let _: PressStateCallback = self.take_signal(signal);
        R::default()
    }
}

impl<T> SignalReciever<MouseLeftDownSignal, PressStateCallback> for Button<T> {
    fn take_signal(&mut self, _signal: &mut MouseLeftDownSignal) -> PressStateCallback {
        match self.press_state {
            PressState::None | PressState::Pressed => PressStateCallback::NoChange,
            PressState::Hovered => {
                self.press_state = PressState::Pressed;
                PressStateCallback::Update(self.press_state)
            }
        }
    }
}

impl<T, R: Default> SignalReciever<MouseLeftDownSignal, R> for Button<T> {
    fn take_signal(&mut self, signal: &mut MouseLeftDownSignal) -> R {
        let _: PressStateCallback = self.take_signal(signal);
        R::default()
    }
}

impl<T> SignalReciever<MouseLeftUpSignal, ClickedCallback> for Button<T> {
    fn take_signal(&mut self, _signal: &mut MouseLeftUpSignal) -> ClickedCallback {
        if self.press_state == PressState::Pressed {
            self.press_state = PressState::Hovered; // callback reciever should know this changed based on the ClickedCallback
            ClickedCallback::Clicked
        } else {
            ClickedCallback::NoClick
        }
    }
}

impl<T, R: Default> SignalReciever<MouseLeftUpSignal, R> for Button<T> {
    fn take_signal(&mut self, signal: &mut MouseLeftUpSignal) -> R {
        let _: ClickedCallback = self.take_signal(signal);
        R::default()
    }
}

impl<T, R: Default> SignalReciever<CharacterInputSignal, R> for Button<T> {
    fn take_signal(&mut self, _signal: &mut CharacterInputSignal) -> R {
        R::default()
    }
}

pub enum PressStateCallback {
    NoChange,
    Update(PressState)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClickedCallback {
    NoClick,
    Clicked
}
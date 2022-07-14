use std::marker::PhantomData;

pub trait SignalReciever<T, R> {
    fn take_signal(&mut self, signal: &mut T) -> R;
}

pub struct ResizedSignal {
    pub resxp: u32,
    pub resyp: u32,
    pub resx: f32,
    pub resy: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CursorMovedSignal {
    pub pixel_posx: f32,
    pub pixel_posy: f32,
    pub norm_posx: f32,
    pub norm_posy: f32,
}

pub struct MouseLeftDownSignal();
pub struct MouseLeftUpSignal();

pub struct CharacterInputSignal {
    pub input: char,
}

#[derive(Debug, Clone, Copy)]
pub struct ReconstructCallback<T> {
    reconstruct: bool,
    phantom: PhantomData<T>,
}

impl<T> ReconstructCallback<T> {
    pub fn new(reconstruct: bool) -> Self {
        Self {
            reconstruct,
            phantom: PhantomData,
        }
    }

    pub fn or(&mut self, reconstruct_callback: ReconstructCallback<T>) {
        if reconstruct_callback.reconstruct {
            self.reconstruct = true;
        }
    }

    pub fn get_reconstruct(&self) -> bool {
        self.reconstruct
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RedrawCallback {
    redraw: bool,
}

impl RedrawCallback {
    pub fn new(redraw: bool) -> Self {
        Self {
            redraw,
        }
    }

    pub fn or(&mut self, redraw_callback: RedrawCallback) {
        if redraw_callback.get_redraw() {
            self.redraw = true;
        }
    }

    pub fn get_redraw(&self) -> bool {
        self.redraw
    }
}
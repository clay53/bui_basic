use std::marker::PhantomData;

pub trait SignalReciever<T, R> {
    fn take_signal(&mut self, signal: &mut T) -> R;
}

// pub struct SignalInterceptor<Child, Signal, Callback> {
//     child: Child,
//     function: fn(&mut Child, &mut Signal) -> Callback,
//     signal: PhantomData<Signal>,
//     callback: PhantomData<Callback>,
// }

// impl<Child, Signal, Callback> SignalInterceptor<Child, Signal, Callback> {
//     pub const fn new(child: Child, function: fn(&mut Child, &mut Signal) -> Callback) -> Self {
//         Self {
//             child,
//             function,
//             signal: PhantomData,
//             callback: PhantomData,
//         }
//     }
// }

// impl<Child, Signal, Callback> SignalReciever<Signal, Callback> for SignalInterceptor<Child, Signal, Callback> {
//     fn take_signal(&mut self, signal: &mut Signal) -> Callback {
//         (self.function)(&mut self.child, signal)
//     }
// }

// impl<Child: SignalReciever<FSignal, FCallback>, Signal, Callback, FSignal, FCallback> SignalReciever<FSignal, FCallback> for SignalInterceptor<Child, Signal, Callback> {
//     fn take_signal(&mut self, signal: &mut FSignal) -> FCallback {
//         self.child.take_signal(signal)
//     }
// }

// would be a lot cleaner with: https://github.com/rust-lang/rust/pull/49624

// pub struct SignalCallbackCatcher<Child: SignalReciever<Signal, OriginalCallback>, Signal, OriginalCallback, TargetCallback> {
//     child: Child,
//     convert_fn: fn(OriginalCallback) -> TargetCallback,
//     signal: PhantomData<Signal>,
//     original_callback: PhantomData<OriginalCallback>,
//     target_callback: PhantomData<TargetCallback>,
// }

// impl<Child: SignalReciever<Signal, OriginalCallback>, Signal, OriginalCallback, TargetCallback> SignalCallbackCatcher<Child, Signal, OriginalCallback, TargetCallback> {
//     pub const fn new(child: Child, convert_fn: fn(OriginalCallback) -> TargetCallback) -> Self {
//         Self {
//             child,
//             convert_fn,
//             signal: PhantomData,
//             original_callback: PhantomData,
//             target_callback: PhantomData,
//         }
//     }
// }

// impl<Child: SignalReciever<Signal, OriginalCallback>, Signal, OriginalCallback, TargetCallback> SignalReciever<Signal, TargetCallback> for SignalCallbackCatcher<Child, Signal, OriginalCallback, TargetCallback> {
//     fn take_signal(&mut self, signal: &mut Signal) -> TargetCallback {
//         (self.convert_fn)(self.child.take_signal(signal))
//     }
// }

// impl<T: SignalReciever<S, A>, S, A, O, R> SignalReciever<S, A> for SignalCallbackCatcher<T, S, O, R> {
//     fn take_signal(&mut self, signal: &mut S) -> A {
//         self.child.take_signal(signal)
//     }
// }

pub trait ShortCircuitingCallback {
    fn or_into(self, other: Self) -> Self;
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

pub struct ScrollSignal {
    pub px: f32,
    pub py: f32,
}

pub struct CharacterInputSignal {
    pub input: char,
}

#[derive(Debug, Clone, Copy)]
pub struct ReconstructCallback<T> {
    reconstruct: bool,
    phantom: PhantomData<T>,
}

impl<T> ReconstructCallback<T> {
    pub const fn new(reconstruct: bool) -> Self {
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

impl<T> Default for ReconstructCallback<T> {
    fn default() -> Self {
        ReconstructCallback::new(false)
    }
}

impl<T> ShortCircuitingCallback for ReconstructCallback<T> {
    fn or_into(mut self, other: Self) -> Self {
        self.or(other);
        self
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
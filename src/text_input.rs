use std::sync::{Arc, Mutex};

use bui::{ttf::CachedFace, rect::Points, line::LineRaw};
use log::info;

use crate::{text::Text, signal::{SignalReciever, CursorMovedSignal, ResizedSignal, ReconstructCallback, MouseLeftUpSignal, CharacterInputSignal}, construct::{LineTarget, Construct, StandardConstructTarget}, containers::{Fill, GetPointBounds}};

#[derive(Debug, Clone)]
pub struct TextInput {
    text: Text,
    bounds: Points,
    hovered: bool,
    focused: bool,
}

impl TextInput {
    pub fn new<T: Into<String>>(text: T, face: Arc<Mutex<CachedFace>>) -> Self {
        Self::new_with_res(text, face, 1.0, 1.0)
    }

    pub fn new_with_res<T: Into<String>>(text: T, face: Arc<Mutex<CachedFace>>, resx: f32, resy: f32) -> Self {
        Self {
            text: Text::new_with_res(text, face, resx, resy),
            bounds: Points::ZERO,
            hovered: false,
            focused: false,
        }
    }

    pub fn get_text(&self) -> &String {
        self.text.get_text()
    }
}

impl Construct<LineTarget> for TextInput {
    fn construct(&self) -> LineTarget {
        let mut target = self.text.construct();
        let border_lines = vec![
            LineRaw {
                p1: [self.bounds.p1x, self.bounds.p2y],
                p2: [self.bounds.p1x, self.bounds.p1y],
            },
            LineRaw {
                p1: [self.bounds.p1x, self.bounds.p1y],
                p2: [self.bounds.p2x, self.bounds.p1y],
            },
            LineRaw {
                p1: [self.bounds.p2x, self.bounds.p1y],
                p2: [self.bounds.p2x, self.bounds.p2y],
            },
            LineRaw {
                p1: [self.bounds.p2x, self.bounds.p2y],
                p2: [self.bounds.p1x, self.bounds.p2y],
            }
        ];
        target.append(LineTarget(border_lines));
        target
    }
}

impl Fill for TextInput {
    fn fill(&mut self, fill_target: bui::rect::SizeAndCenter) {
        self.text.fill(fill_target);
        self.bounds = self.text.get_point_bounds();
    }
}

impl SignalReciever<ResizedSignal, ReconstructCallback<LineTarget>> for TextInput {
    fn take_signal(&mut self, signal: &mut ResizedSignal) -> ReconstructCallback<LineTarget> {
        self.text.take_signal(signal)
    }
}

impl SignalReciever<CursorMovedSignal, ()> for TextInput {
    fn take_signal(&mut self, signal: &mut CursorMovedSignal) -> () {
        self.hovered = self.bounds.contains(signal.norm_posx, signal.norm_posy);
    }
}

impl SignalReciever<MouseLeftUpSignal, ()> for TextInput {
    fn take_signal(&mut self, _signal: &mut MouseLeftUpSignal) -> () {
        self.focused = self.hovered;
        if self.focused {
            // show keyboard
            #[cfg(target_os="android")]
            {
                let ctx = ndk_context::android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                let env = vm.attach_current_thread().unwrap();
                
                let context_class = env.find_class("android/content/Context").unwrap();
                let input_method_service_name = env.get_static_field(context_class, "INPUT_METHOD_SERVICE", "Ljava/lang/String;").unwrap();

                let input_method_manager = env.call_method(ctx.context().cast(), "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[input_method_service_name]).unwrap().l().unwrap();
                
                env.call_method(input_method_manager, "toggleSoftInput", "(II)V", &[2.into(), 0.into()]).unwrap();
            }
        }
    }
}

impl SignalReciever<CharacterInputSignal, ReconstructCallback<LineTarget>> for TextInput {
    fn take_signal(&mut self, signal: &mut CharacterInputSignal) -> ReconstructCallback<LineTarget> {
        if self.focused {
            match signal.input {
                '\u{8}' => self.text.backspace(),
                _ => self.text.set_text(format!("{}{}", self.text.get_text(), signal.input))
            }
            ReconstructCallback::new(true)
        } else {
            ReconstructCallback::new(false)
        }
    }
}
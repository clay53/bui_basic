use std::sync::{Arc, Mutex};

use bui::{ttf::CachedFace, rect::Points, line::LineRaw};

use crate::{text::Text, signal::{SignalReciever, CursorMovedSignal, ResizedSignal, ReconstructCallback, MouseLeftUpSignal, CharacterInputSignal, MouseLeftDownSignal}, construct::{LineTarget, Construct, StandardConstructTarget}, containers::{Fill, GetPointBounds, FillWidth, TranslateY, TranslateX, Init}};

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

impl Init for TextInput {
    fn init(&mut self) {}
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

impl FillWidth for TextInput {
    fn fill_width(&mut self, _sx: f32, _cx: f32, _ty: f32) -> f32 {
        todo!()
    }
}

impl TranslateX for TextInput {
    fn translate_x(&mut self, dx: f32) {
        self.text.translate_x(dx);
        self.bounds.p1x += dx;
        self.bounds.p2x += dx;
    }
}

impl TranslateY for TextInput {
    fn translate_y(&mut self, dy: f32) {
        self.text.translate_y(dy);
        self.bounds.p1y += dy;
        self.bounds.p2y += dy;
    }
}

impl SignalReciever<ResizedSignal, ReconstructCallback<LineTarget>> for TextInput {
    fn take_signal(&mut self, signal: &mut ResizedSignal) -> ReconstructCallback<LineTarget> {
        self.text.take_signal(signal)
    }
}

impl<R: Default> SignalReciever<CursorMovedSignal, R> for TextInput {
    fn take_signal(&mut self, signal: &mut CursorMovedSignal) -> R {
        self.hovered = self.bounds.contains(signal.norm_posx, signal.norm_posy);
        R::default()
    }
}

pub enum TextInputFocusCallback {
    None,
    Unfocused,
    Focused,
}

impl SignalReciever<MouseLeftUpSignal, TextInputFocusCallback> for TextInput {
    fn take_signal(&mut self, _signal: &mut MouseLeftUpSignal) -> TextInputFocusCallback {
        self.focused = self.hovered;
        if self.hovered {
            if self.focused {
                TextInputFocusCallback::None
            } else {
                self.focused = true;
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
                TextInputFocusCallback::Focused
            }
        } else {
            if self.focused {
                self.focused = false;
                TextInputFocusCallback::Unfocused
            } else {
                TextInputFocusCallback::None
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

impl<R: Default> SignalReciever<MouseLeftDownSignal, R> for TextInput {
    fn take_signal(&mut self, _signal: &mut MouseLeftDownSignal) -> R {
        R::default()
    }
}
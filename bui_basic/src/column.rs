use crate::{construct::{ Construct, StandardConstructTarget }, containers::{Init, GetHeight, SetCenterTopPosition, Fill, FillWidth, TranslateX, TranslateY}, signal::{SignalReciever, ReconstructCallback}};

#[derive(Debug, Clone)]
pub struct VecColumn<T> {
    children: Vec<T>,
    cx: f32,
    ty: f32,
}

impl<T> VecColumn<T> {
    pub fn new() -> Self {
        VecColumn {
            children: Vec::new(),
            cx: 0.0,
            ty: 1.0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.children.push(value);
    }

    pub fn into_push(mut self, value: T) -> Self {
        self.push(value);
        self
    }

    pub fn set_top_y(&mut self, top_y: f32) {
        self.ty = top_y;
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<T> {
        &mut self.children
    }

    pub fn reposition_children(&mut self)
    where
        T: GetHeight+SetCenterTopPosition
    {
        let mut y = self.ty;
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
            ty: 0.0
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

// impl<T: GetHeight+SetCenterTopPosition+Init> Init for VecColumn<T> {
impl<T: Init> Init for VecColumn<T> {
    fn init(&mut self) {
        // let mut ty = self.ty;
        for child in &mut self.children {
            // child.set_center_top_position(self.cx, ty);
            // ty -= child.get_height();
            child.init();
        }
    }
}

impl<T: FillWidth> FillWidth for VecColumn<T> {
    fn fill_width(&mut self, sx: f32, cx: f32, ty: f32) -> f32 {
        let mut next_ty = ty;
        for child in &mut self.children {
            next_ty = child.fill_width(sx, cx, next_ty)
        }
        next_ty
    }
}

impl<T: TranslateX> TranslateX for VecColumn<T> {
    fn translate_x(&mut self, dx: f32) {
        self.cx += dx;
        for child in &mut self.children {
            child.translate_x(dx);
        }
    }
}

impl<T: TranslateY> TranslateY for VecColumn<T> {
    fn translate_y(&mut self, dy: f32) {
        self.ty += dy;
        for child in &mut self.children {
            child.translate_y(dy)
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

impl<T: SignalReciever<S, ReconstructCallback<C>>, S, C> SignalReciever<S, ReconstructCallback<C>> for VecColumn<T> {
    fn take_signal(&mut self, signal: &mut S) -> ReconstructCallback<C> {
        let reconstruct_callbacks: Vec<ReconstructCallback<C>> = self.take_signal(signal);
        for reconstruct_callback in reconstruct_callbacks {
            if reconstruct_callback.get_reconstruct() {
                return reconstruct_callback
            }
        }
        ReconstructCallback::new(false)
    }
}
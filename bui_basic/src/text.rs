use std::sync::{Arc, Mutex};

use bui::{ttf::CachedFace, ttf_outline::{compute_unfit_chars, compute_square_transform, transform_lines, transform_points, transform_points_vec, compute_square_transform_by_width}, rect::{SizeAndCenter, Points}, line::LineRaw};

use crate::{construct::{LineTarget, Construct}, containers::{Fill, GetPointBounds, GetCenterPosition, FillWidth, TranslateY, TranslateX, Init}, signal::{SignalReciever, ResizedSignal, CursorMovedSignal, MouseLeftDownSignal, MouseLeftUpSignal, ReconstructCallback, CharacterInputSignal}};

#[derive(Debug, Clone)]
pub enum TextSizeMode {
    Unconstrained,
    Fill(SizeAndCenter),
    FillWidth(f32, f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragPoint {
    char_index: usize,
    char_side: CharSide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DragState {
    pub start: DragPoint,
    pub end: DragPoint
}

impl DragState {
    pub fn compute_selection(&self) -> Option<Selection> {
        let start = if self.start.char_side == CharSide::Left {
            self.start.char_index
        } else {
            self.start.char_index+1
        };

        let end = if self.end.char_side == CharSide::Left {
            self.end.char_index
        } else {
            self.end.char_index+1
        };

        if start == end {
            None
        } else {
            Some(Selection {
                start,
                end,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectState {
    None,
    Dragging(DragState, Option<Selection>),
    Selected(Selection)
}

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    face: Arc<Mutex<CachedFace>>,
    size_mode: TextSizeMode,
    resx: f32,
    resy: f32,
    lines: Option<Vec<LineRaw>>,
    chars_bounds: Option<Points>,
    char_bounds: Option<Vec<Points>>,
    mousex: f32,
    mousey: f32,
    select_state: SelectState,
    selection_lines: Option<[LineRaw; 4]>,
}

impl Text {
    pub fn new<T: Into<String>>(text: T, face: Arc<Mutex<CachedFace>>) -> Self {
        Self::new_with_res(text.into(), face, 1.0, 1.0)
    }

    pub fn new_with_res<T: Into<String>>(text: T, face: Arc<Mutex<CachedFace>>, resx: f32, resy: f32) -> Self {
        Text {
            text: text.into(),
            face,
            size_mode: TextSizeMode::Unconstrained,
            resx,
            resy,
            lines: None,
            chars_bounds: None,
            char_bounds: None,
            mousex: -1.0,
            mousey: -1.0,
            select_state: SelectState::None,
            selection_lines: None,
        }
    }

    fn compute_chars(&mut self) {
        let (lines, chars_bounds, char_bounds) = match self.size_mode {
            TextSizeMode::Unconstrained => todo!(),
            TextSizeMode::Fill(fill_target) => {
                let (mut lines, mut chars_bounds, mut char_bounds) = compute_unfit_chars(&mut self.face.lock().unwrap(), self.text.as_str(), 5);
                let transform = compute_square_transform(chars_bounds, fill_target, self.resx, self.resy);
                transform_lines(&mut lines, transform);
                transform_points(&mut chars_bounds, transform);
                transform_points_vec(&mut char_bounds, transform);

                (Some(lines), Some(chars_bounds), Some(char_bounds))
            },
            TextSizeMode::FillWidth(sx, cx, ty) => {
                let (mut lines, mut chars_bounds, mut char_bounds) = compute_unfit_chars(&mut self.face.lock().unwrap(), self.text.as_str(), 5);
                let transform = compute_square_transform_by_width(chars_bounds, sx, cx, ty, self.resx, self.resy);
                transform_lines(&mut lines, transform);
                transform_points(&mut chars_bounds, transform);
                transform_points_vec(&mut char_bounds, transform);

                (Some(lines), Some(chars_bounds), Some(char_bounds))
            }
        };

        self.lines = lines;
        self.chars_bounds = chars_bounds;
        self.char_bounds = char_bounds;
    }

    fn pos_to_select_point(&self, x: f32, y: f32) -> Option<DragPoint> {
        match &self.char_bounds {
            Some(char_bounds) => {
                for (i, points) in char_bounds.iter().enumerate() {
                    if points.left_contains(x, y) {
                        return Some(DragPoint {
                            char_index: i,
                            char_side: CharSide::Left
                        })
                    } else if points.right_contains(x, y) {
                        return Some(DragPoint {
                            char_index: i,
                            char_side: CharSide::Right
                        })
                    }
                }
                None
            },
            None => None
        }
    }

    fn selection_to_lines(&self, selection: &Selection) -> [LineRaw; 4] {        
        let chars_bounds = self.chars_bounds.unwrap();
        let ymax = chars_bounds.p1y;
        let ymin = chars_bounds.p2y;

        let char_bounds = self.char_bounds.as_ref().unwrap();

        let (xmin, xmax) = if selection.start < selection.end {
            (char_bounds[selection.start].p1x, char_bounds[selection.end-1].p2x)
        } else {
            (char_bounds[selection.end].p1x, char_bounds[selection.start-1].p2x)
        };

        [
            LineRaw {
                p1: [xmin, ymin],
                p2: [xmin, ymax],
            },
            LineRaw {
                p1: [xmin, ymax],
                p2: [xmax, ymax],
            },
            LineRaw {
                p1: [xmax, ymax],
                p2: [xmax, ymin],
            },
            LineRaw {
                p1: [xmax, ymin],
                p2: [xmin, ymin],
            }
        ]
    }

    fn compute_selection_lines(&mut self) {
        let selection_lines = match &self.select_state {
            SelectState::None => None,
            SelectState::Dragging(_, selection) => match selection {
                Some(selection) => Some(self.selection_to_lines(selection)),
                None => None,
            },
            SelectState::Selected(selection) => Some(self.selection_to_lines(selection))
        };
        self.selection_lines = selection_lines;
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
        
        if let Some(_) = self.lines {
            self.compute_chars();
        }
        if self.select_state != SelectState::None {
            self.select_state = SelectState::None;
            self.compute_selection_lines();
        }
    }

    pub fn backspace(&mut self) {
        self.text.pop();

        if let Some(_) = self.lines {
            self.compute_chars();
        }
        if self.select_state != SelectState::None {
            self.select_state = SelectState::None;
            self.compute_selection_lines();
        }
    }
}

impl Construct<LineTarget> for Text {
    fn construct(&self) -> LineTarget {
        LineTarget(
            match &self.lines {
                Some(lines) => {
                    let mut lines = lines.clone();
                    if let Some(selection_lines) = self.selection_lines {
                        lines.append(&mut Vec::from(selection_lines));
                    }
                    lines
                },
                None => Vec::with_capacity(0)
            }
        )
    }
}

impl Init for Text {
    fn init(&mut self) {}
}

impl Fill for Text {
    fn fill(&mut self, fill_target: SizeAndCenter) {
        self.size_mode = TextSizeMode::Fill(fill_target);
        self.compute_chars();
        self.compute_selection_lines();
    }
}

impl FillWidth for Text {
    fn fill_width(&mut self, sx: f32, cx: f32, ty: f32) -> f32 {
        todo!()
    }
}

impl TranslateX for Text {
    fn translate_x(&mut self, dx: f32) {
        match &mut self.size_mode {
            TextSizeMode::Fill(size_and_center) => {
                size_and_center.cx += dx;

                if let Some(chars_bounds) = self.chars_bounds.as_mut() {
                    chars_bounds.p1x += dx;
                    chars_bounds.p2x += dx;
                }

                if let Some(lines) = self.lines.as_mut() {
                    for line in lines {
                        line.p1[0] += dx;
                        line.p2[0] += dx;
                    }
                }
                
                if let Some(chars_bounds) = self.char_bounds.as_mut() {
                    for point in chars_bounds {
                        point.p1x += dx;
                        point.p2x += dx;
                    }
                }
                
                if let Some(selection_lines) = self.selection_lines.as_mut() {
                    for line in selection_lines {
                        line.p1[0] += dx;
                        line.p2[0] += dx;
                    }
                }
            },
            TextSizeMode::FillWidth(_, _, _) => todo!(),
            TextSizeMode::Unconstrained => todo!(),
        }
    }
}

impl TranslateY for Text {
    fn translate_y(&mut self, dy: f32) {
        match &mut self.size_mode {
            TextSizeMode::Fill(size_and_center) => {
                size_and_center.cy += dy;

                if let Some(chars_bounds) = self.chars_bounds.as_mut() {
                    chars_bounds.p1y += dy;
                    chars_bounds.p2y += dy;
                }

                if let Some(lines) = self.lines.as_mut() {
                    for line in lines {
                        line.p1[1] += dy;
                        line.p2[1] += dy;
                    }
                }
                
                if let Some(chars_bounds) = self.char_bounds.as_mut() {
                    for point in chars_bounds {
                        point.p1y += dy;
                        point.p2y += dy;
                    }
                }
                
                if let Some(selection_lines) = self.selection_lines.as_mut() {
                    for line in selection_lines {
                        line.p1[1] += dy;
                        line.p2[1] += dy;
                    }
                }
            },
            TextSizeMode::FillWidth(_, _, _) => todo!(),
            TextSizeMode::Unconstrained => todo!(),
        }
    }
}

impl SignalReciever<ResizedSignal, ReconstructCallback<LineTarget>> for Text {
    fn take_signal(&mut self, signal: &mut ResizedSignal) -> ReconstructCallback<LineTarget> {
        self.resx = signal.resx;
        self.resy = signal.resy;
        self.compute_chars();
        self.compute_selection_lines();
        ReconstructCallback::new(true)
    }
}

impl SignalReciever<CursorMovedSignal, (ReconstructCallback<LineTarget>, SelectStateCallback)> for Text {
    fn take_signal(&mut self, signal: &mut CursorMovedSignal) -> (ReconstructCallback<LineTarget>, SelectStateCallback) {
        self.mousex = signal.norm_posx;
        self.mousey = signal.norm_posy;

        match self.pos_to_select_point(self.mousex, self.mousey) {
            Some(select_point) => {
                match &mut self.select_state {
                    SelectState::Dragging(drag_state, selection) => {
                        if drag_state.end != select_point {
                            drag_state.end = select_point;
                            let new_selection = drag_state.compute_selection();
                            if new_selection != *selection {
                                *selection = new_selection;
                                self.compute_selection_lines();
                                (ReconstructCallback::new(true), SelectStateCallback::Update(self.select_state.clone()))
                            } else {
                                (ReconstructCallback::new(true), SelectStateCallback::Update(self.select_state.clone()))
                            }
                        } else {
                            (ReconstructCallback::new(false), SelectStateCallback::NoChange)
                        }
                    },
                    _ => (ReconstructCallback::new(false), SelectStateCallback::NoChange)
                }
            },
            None => (ReconstructCallback::new(false), SelectStateCallback::NoChange)
        }
    }
}

impl SignalReciever<CursorMovedSignal, ReconstructCallback<LineTarget>> for Text {
    fn take_signal(&mut self, signal: &mut CursorMovedSignal) -> ReconstructCallback<LineTarget> {
        let (reconstruct_callback, _): (ReconstructCallback<LineTarget>, SelectStateCallback) = self.take_signal(signal);
        reconstruct_callback
    }
}

impl SignalReciever<MouseLeftDownSignal, (ReconstructCallback<LineTarget>, SelectStateCallback)> for Text {
    fn take_signal(&mut self, _signal: &mut MouseLeftDownSignal) -> (ReconstructCallback<LineTarget>, SelectStateCallback) {
        match self.pos_to_select_point(self.mousex, self.mousey) {
            Some(drag_point) => {
                let drag_state = DragState {
                    start: drag_point,
                    end: drag_point
                };
                let selection = drag_state.compute_selection();
                self.select_state = SelectState::Dragging(drag_state, selection);
                self.compute_selection_lines();
                (ReconstructCallback::new(true), SelectStateCallback::Update(self.select_state.clone()))
            },
            None => {
                if self.select_state == SelectState::None {
                    (ReconstructCallback::new(false), SelectStateCallback::NoChange)
                } else {
                    self.select_state = SelectState::None;
                    self.compute_selection_lines();
                    (ReconstructCallback::new(true), SelectStateCallback::Update(self.select_state.clone()))
                }
            }
        }
    }
}

impl SignalReciever<MouseLeftDownSignal, ReconstructCallback<LineTarget>> for Text {
    fn take_signal(&mut self, signal: &mut MouseLeftDownSignal) -> ReconstructCallback<LineTarget> {
        let (reconstruct_callback, _) = self.take_signal(signal);
        reconstruct_callback
    }
}

impl SignalReciever<MouseLeftUpSignal, SelectStateCallback> for Text {
    fn take_signal(&mut self, _signal: &mut MouseLeftUpSignal) -> SelectStateCallback {
        match &self.select_state {
            SelectState::None | SelectState::Selected(_) => SelectStateCallback::NoChange,
            SelectState::Dragging(_, selection) => {
                match selection {
                    Some(selection) => self.select_state = SelectState::Selected(selection.clone()),
                    None => self.select_state = SelectState::None
                }
                SelectStateCallback::Update(self.select_state.clone())
            }
        }
    }
}

impl<R: Default> SignalReciever<MouseLeftUpSignal, R> for Text {
    fn take_signal(&mut self, signal: &mut MouseLeftUpSignal) -> R {
        let _: SelectStateCallback = self.take_signal(signal);
        R::default()
    }
}

impl<R: Default> SignalReciever<CharacterInputSignal, R> for Text {
    fn take_signal(&mut self, _signal: &mut CharacterInputSignal) -> R {
        R::default()
    }
}

#[derive(Debug, Clone)]
pub enum SelectStateCallback {
    NoChange,
    Update(SelectState),
}

impl GetPointBounds for Text {
    fn get_point_bounds(&self) -> Points {
        match self.size_mode {
            TextSizeMode::Fill(fill_target) => fill_target.into(),
            TextSizeMode::FillWidth(sx, cx, ty) => todo!(),
            TextSizeMode::Unconstrained => Points::ZERO,
        }
    }
}

impl GetCenterPosition for Text {
    fn get_center_position(&self) -> (f32, f32) {
        match self.size_mode {
            TextSizeMode::Fill(fill) => (fill.cx, fill.cy),
            TextSizeMode::FillWidth(sx, cx, ty) => todo!(),
            TextSizeMode::Unconstrained => todo!()
        }
    }
}
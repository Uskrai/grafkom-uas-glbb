use std::{
    ops::RangeInclusive,
    time::{Duration, Instant},
};

use egui::{pos2, Response, Sense, TextureHandle};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GLBBState {
    pub pos: egui::Pos2,
    pub original_radius: f32,
    pub size: egui::Vec2,

    pub horizontal: HorizontalState,
    pub vertical: VerticalState,

    #[serde(skip)]
    pub circle_texture: Option<TextureHandle>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct HorizontalState {
    play: Option<i8>,
    #[serde(skip)]
    pub start: Now,
    #[serde(skip)]
    pub duration: Duration,

    pub velocity: f64,
    pub acceleration: f64,
}

fn calculate_distance(velocity: f64, acceleration: f64, time: f64) -> f64 {
    velocity * time + (0.5) * -acceleration * (time * time)
}

fn calculate_velocity(velocity: f64, acceleration: f64, time: f64) -> f64 {
    velocity + -acceleration * time
}

impl HorizontalState {
    pub fn is_play(&self) -> bool {
        self.play.is_some()
    }
    pub fn stop(&mut self) {
        self.play = None;
    }

    pub fn play_left(&mut self) {
        self.play(-1);
    }

    pub fn play_right(&mut self) {
        self.play(1);
    }

    fn play(&mut self, direction: i8) {
        self.start.reset();
        self.play = Some(direction);
        let duration = self.velocity.abs() / self.acceleration.abs();
        self.duration = Duration::from_secs_f64(duration);
    }

    pub fn distance_at(&self, time: f64) -> f64 {
        calculate_distance(self.velocity, self.acceleration, time)
    }

    pub fn velocity_at(&self, time: f64) -> f64 {
        calculate_velocity(self.velocity, self.acceleration, time)
    }

    pub fn mv(&mut self, pos: &mut f32, range: RangeInclusive<f32>) {
        if let Some(mut direction) = self.play {
            let time = self.start.elapsed().min(self.duration).as_secs_f64();
            let mut distance = self.distance_at(time);

            while distance > 0.0 {
                let move_by = distance.min(5.0);
                distance -= move_by;
                *pos += (move_by as f32) * (direction as f32);

                if !range.contains(pos) {
                    direction *= -1;
                }
            }

            self.velocity = self.velocity_at(time);
            self.play = Some(direction);

            if self.start.elapsed() < self.duration {
                self.play(direction);
            } else {
                self.stop();
            }
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct VerticalState {
    play: bool,
    #[serde(skip)]
    start: Now,
    pub is_drop: bool,
    pub accel: f32,
}

impl VerticalState {
    pub fn fall(&mut self) {
        self.accel = 10.0;
        self.play = true;
        self.is_drop = true;
        self.start.reset();
    }

    pub fn stop(&mut self) {
        self.play = false;
    }

    pub fn mv(&mut self, pos: &mut f32, _: f32) {
        if self.play {
            print!("{:?} ", self.start.elapsed().as_secs_f64());

            if self.is_drop {
                self.accel += 9.8;
                self.accel += self.accel * self.start.elapsed().as_secs_f32();
                *pos -= self.accel;

                if *pos <= 0.0 {
                    self.is_drop = false;
                    *pos = 0.0;
                }
            } else {
                self.accel -= 9.8;
                self.accel -= self.accel * self.start.elapsed().as_secs_f32();
                *pos += self.accel;

                if self.accel <= 0.0 {
                    self.is_drop = true;
                }
            }
            self.start.reset();

            if self.accel <= 0.0 && *pos <= 0.0 {
                self.play = false;
            }
        }
    }

    pub fn is_play(&self) -> bool {
        self.play
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GLBBDirection {
    Stop,
    Vertical(i8),
    Horizontal(i8),
}

impl GLBBDirection {
    pub fn set_direction(&mut self, new: i8) {
        match self {
            Self::Vertical(i) | Self::Horizontal(i) => {
                *i = new;
            }
            Self::Stop => {}
        }
    }

    /// Returns `true` if the glbbdirection is [`Stop`].
    ///
    /// [`Stop`]: GLBBDirection::Stop
    #[must_use]
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }
}

// #[derive(Debug)]
pub struct Now(Instant);

impl std::fmt::Debug for Now {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Now({:?})", self.elapsed())
    }
}

impl Default for Now {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Now {
    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }

    pub fn reset(&mut self) {
        self.0 = Instant::now();
    }
}

impl Default for GLBBDirection {
    fn default() -> Self {
        Self::Stop
    }
}

impl GLBBDirection {
    pub fn stop() -> Self {
        Self::Stop
    }

    pub fn left() -> Self {
        Self::Horizontal(-1)
    }

    pub fn right() -> Self {
        Self::Horizontal(1)
    }

    pub fn up() -> Self {
        Self::Vertical(1)
    }

    pub fn down() -> Self {
        Self::Vertical(-1)
    }
}

impl GLBBState {
    pub fn pos_to_screen(&self, rect: egui::Rect) -> egui::Pos2 {
        let radius = self.radius_size();
        let x = self.pos.x + rect.min.x + radius.x;
        let y = rect.max.y - radius.y - self.pos.y - 2.0;

        egui::pos2(x, y)
    }

    pub fn pos_from_screen(&self, rect: egui::Rect, pos: egui::Pos2) -> egui::Pos2 {
        let radius = self.radius_size();
        let x = pos.x - rect.min.x - radius.x;
        let y = rect.max.y - pos.y - radius.y;

        egui::pos2(x, y)
    }

    pub fn is_play(&self) -> bool {
        self.horizontal.is_play() || self.vertical.is_play()
    }

    pub fn clamp(&mut self) {
        self.pos = self
            .pos
            .to_vec2()
            .clamp(egui::Vec2::ZERO, self.pos_max())
            .to_pos2();
    }

    pub fn radius(&self) -> f32 {
        let y = self.pos.y.max(0.0) / (self.size.y.max(0.0));
        let min_by = 0.5 * y;
        let scale = 1.0 - min_by.min(0.9);
        self.original_radius * scale
    }

    pub fn pos_x_max(&self) -> f32 {
        self.size.x - (self.radius() * 2.0)
    }

    pub fn pos_max(&self) -> egui::Vec2 {
        (self.size - (self.radius_size() * 2.0)).max(egui::vec2(1f32, 1f32))
    }

    pub fn pos_y_max(&self) -> f32 {
        self.size.y - (self.radius() * 2.0)
    }

    pub fn radius_size(&self) -> egui::Vec2 {
        [self.radius(), self.radius()].into()
    }
}

pub struct GLBBWidget<'a> {
    state: &'a mut GLBBState,
    id: Option<egui::Id>,
}

impl<'a> GLBBWidget<'a> {
    pub fn new(state: &'a mut GLBBState) -> Self {
        Self { state, id: None }
    }

    pub fn id_source(mut self, id: impl std::hash::Hash) {
        self.id = Some(egui::Id::new(id));
    }

    pub fn show(mut self, ui: &mut egui::Ui) -> Response {
        let Self { state, .. } = &mut self;

        let response =
            ui.allocate_response(ui.available_size_before_wrap(), Sense::click_and_drag());
        state.size = response.rect.size();

        let painter = ui.painter_at(response.rect);

        painter.rect(
            // egui::Rect::from_min_size(pos2(0.0, 0.0), response.rect.size()),
            response.rect,
            egui::Rounding::none(),
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );

        state.clamp();
        let max = state.pos_max();

        state.horizontal.mv(&mut state.pos.x, 0.0..=max.x);
        state.vertical.mv(&mut state.pos.y, max.y);

        if state.vertical.play || state.horizontal.is_play() {
            ui.ctx().request_repaint();
        }
        state.clamp();

        self.draw_circle(ui, response.rect);

        response
    }

    fn draw_circle(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        let Self { state, .. } = self;
        let center_pos = state.pos_to_screen(rect);

        let painter = ui.painter_at(rect);

        let create_wheel_point = |radius: f32, pos: egui::Pos2, wheel: u32| {
            let wheel_f = wheel as f32;
            (0..(wheel as u32))
                .into_iter()
                .map(|it| it as f32)
                // ngebagi 360 bagian menjadi wheel bagian
                .map(|it| it * 360.0 / wheel_f)
                // membuat derajat relatif dengan posisi
                .map(|it| it + pos.x + pos.y)
                // mengubah derajat menjadi radians
                .map(|it| it.to_radians())
                // menghitung titik sisi dengan kemerengan derajatnya.
                .map(|it| pos2(pos.x + (radius * it.cos()), pos.y + (radius * it.sin())))
                .collect::<Vec<_>>()
        };

        let color = egui::Color32::GOLD;

        let image_texture = match state.circle_texture.clone() {
            Some(texture) => texture,
            None => {
                // let rad = state.radius() as usize;
                let rad = 5000;
                let dia = rad * 2 + 1;
                let mut image = egui::ColorImage::new([dia, dia], egui::Color32::TRANSPARENT);
                let points = mid_point(rad, rad, rad);

                for it in points.into_iter() {
                    image[it] = color;
                }

                let texture = ui.ctx().load_texture("Circle", image);
                state.circle_texture = Some(texture.clone());

                texture
            }
        };
        let stroke = egui::Stroke::new(1.0, egui::Color32::RED);

        let points = create_wheel_point(state.radius(), center_pos, 8);
        for i in 0..points.len() {
            painter.add(egui::Shape::line_segment([center_pos, points[i]], stroke));
        }

        let stroke = egui::Stroke::new(1.0, color);
        let points = create_wheel_point(state.radius(), center_pos, 360);
        for i in 0..points.len() {
            painter.add(egui::Shape::line_segment(
                [points[i], points[(i + 1) % points.len()]],
                stroke,
            ));
        }

        let pos = center_pos - state.radius_size();
        let rect = egui::Rect::from_min_size(pos, state.radius_size() * egui::vec2(2.0, 2.0));
        egui::Image::new(image_texture.id(), state.radius_size()).paint_at(ui, rect);
    }
}

fn mid_point(x_center: usize, y_center: usize, radius: usize) -> Vec<(usize, usize)> {
    let mut x = radius;
    let mut y = 0;
    let mut points = vec![];

    let x_y = |x: usize, y: usize| {
        let x_add = x_center + x;
        let y_add = y_center + y;
        let x_min = x_center.checked_sub(x).unwrap();
        let y_min = y_center.checked_sub(y).unwrap();
        [
            (x_add, y_add),
            (x_min, y_add),
            (x_add, y_min),
            (x_min, y_min),
        ]
    };

    if radius > 0 {
        points.extend(x_y(x, y).iter().take(2).chain(x_y(y, x).iter().take(2)));
    } else {
        points.push(x_y(x, y)[0]);
    }

    let mut p = 1 - radius as isize;
    while x > y {
        y += 1;

        if p <= 0 {
            p = p + 2 * y as isize + 1;
        } else {
            x -= 1;
            p = p + (2 * y as isize) - (2 * x as isize) + 1;
        }

        // all points already added
        if x < y {
            break;
        }

        points.extend(x_y(x, y));
        // points.extend([add_by(x, y), add_by(-x, y), add_by(x, -y), add_by(-x, -y)].iter());

        // kalo x sama dengan y maka titiknya udah ditambahkan
        if x != y {
            points.extend(x_y(y, x));
        }
    }

    points
}

fn point(pos: egui::Pos2, stroke: impl Into<egui::Stroke>) -> egui::Shape {
    egui::Shape::line_segment([pos, pos + egui::vec2(1f32, 1f32)], stroke)
}

use egui::TextureHandle;
use serde::{Deserialize, Serialize};

use crate::{
    horizontal_state::HorizontalState,
    vertical_state::VerticalState,
};

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

impl GLBBState {
    /// translasi posisi dari bola ke layar yang berada di
    /// rect
    pub fn pos_to_screen(
        &self,
        rect: egui::Rect,
    ) -> egui::Pos2 {
        let radius = self.radius_size();
        let x = self.pos.x + rect.min.x + radius.x;
        let y = rect.max.y - radius.y - self.pos.y - 2.0;

        egui::pos2(x, y)
    }

    /// translasi posisi dari layar ke posisi yang berawal dari (0,0)
    pub fn pos_from_screen(
        &self,
        rect: egui::Rect,
        pos: egui::Pos2,
    ) -> egui::Pos2 {
        let radius = self.radius_size();
        let x = pos.x - rect.min.x - radius.x;
        let y = rect.max.y - pos.y - radius.y;

        egui::pos2(x, y)
    }

    /// cek apakah bola sedang bergerak
    pub fn is_play(&self) -> bool {
        self.horizontal.is_play() || self.vertical.is_play()
    }

    /// jepit nilai posisi sehingga tidak melewati layar
    pub fn clamp(&mut self) {
        self.pos = self
            .pos
            .to_vec2()
            .clamp(egui::Vec2::ZERO, self.pos_max())
            .to_pos2();
    }

    /// ambil radius dari bola yang sudah di scale dengan tinggi bola
    pub fn radius(&self) -> f32 {
        let y =
            self.pos.y.max(0.0) / (self.size.y.max(0.0));
        let min_by = 0.5 * y;
        let scale = 1.0 - min_by.min(0.9);
        self.original_radius * scale
    }

    /// posisi maximum dari x
    pub fn pos_x_max(&self) -> f32 {
        self.size.x - (self.radius() * 2.0)
    }

    /// posisi maximum dari y
    pub fn pos_y_max(&self) -> f32 {
        self.size.y - (self.radius() * 2.0)
    }

    /// posisi maximum
    pub fn pos_max(&self) -> egui::Vec2 {
        (self.size - (self.radius_size() * 2.0))
            .max(egui::vec2(1f32, 1f32))
    }

    /// radius dalam bentuk [radius,radius]
    pub fn radius_size(&self) -> egui::Vec2 {
        [self.radius(), self.radius()].into()
    }
}

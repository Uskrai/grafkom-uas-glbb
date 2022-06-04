use egui::{pos2, Response, Sense};

use crate::{mid_point, GLBBState};

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

        let response = ui.allocate_response(
            ui.available_size_before_wrap(),
            Sense::click_and_drag(),
        );
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

        if state.is_play() {
            ui.ctx().request_repaint();
        }
        state.clamp();

        self.draw_circle(ui, response.rect);

        response
    }

    fn draw_circle(
        &mut self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
    ) {
        let Self { state, .. } = self;
        let center_pos = state.pos_to_screen(rect);

        let painter = ui.painter_at(rect);

        let create_wheel_point =
            |radius: f32, pos: egui::Pos2, wheel: u32| {
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
                    .map(|it| {
                        pos2(
                            pos.x + (radius * it.cos()),
                            pos.y + (radius * it.sin()),
                        )
                    })
                    .collect::<Vec<_>>()
            };

        let color = egui::Color32::GOLD;

        let stroke =
            egui::Stroke::new(1.0, egui::Color32::RED);

        let points = create_wheel_point(
            state.radius(),
            center_pos,
            8,
        );
        for i in 0..points.len() {
            painter.add(egui::Shape::line_segment(
                [center_pos, points[i]],
                stroke,
            ));
        }

        let stroke = egui::Stroke::new(1.0, color);
        let points = create_wheel_point(
            state.radius(),
            center_pos,
            360,
        );
        for i in 0..points.len() {
            painter.add(egui::Shape::line_segment(
                [points[i], points[(i + 1) % points.len()]],
                stroke,
            ));
        }

        // let image_texture =
        //     match state.circle_texture.clone() {
        //         Some(texture) => texture,
        //         None => {
        //             // let rad = state.radius() as usize;
        //             let rad = 5000;
        //             let dia = rad * 2 + 1;
        //             let mut image = egui::ColorImage::new(
        //                 [dia, dia],
        //                 egui::Color32::TRANSPARENT,
        //             );
        //             let points = mid_point(rad, rad, rad);

        //             for it in points.into_iter() {
        //                 image[it] = color;
        //             }

        //             let texture = ui
        //                 .ctx()
        //                 .load_texture("Circle", image);
        //             state.circle_texture =
        //                 Some(texture.clone());

        //             texture
        //         }
        //     };
        // let pos = center_pos - state.radius_size();
        // let rect = egui::Rect::from_min_size(
        //     pos,
        //     state.radius_size() * egui::vec2(2.0, 2.0),
        // );
        // egui::Image::new(
        //     image_texture.id(),
        //     state.radius_size(),
        // )
        // .paint_at(ui, rect);
    }
}

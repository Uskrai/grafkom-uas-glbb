use glbb::slider;
use glbb::GLBBState;
use glbb::GLBBWidget;

pub struct App {
    glbb: glbb::GLBBState,
    size: egui::Vec2,
}

impl App {
    pub fn new(it: &eframe::CreationContext) -> Self {
        Self {
            glbb: it
                .storage
                .map(|storage| {
                    storage
                        .get_string("glbb")
                        .map(|it| ron::from_str(&it).ok())
                })
                .flatten()
                .flatten()
                .unwrap_or(GLBBState {
                    original_radius: 30.0,
                    ..Default::default()
                }),
            size: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        _storage.set_string(
            "glbb",
            ron::to_string(&self.glbb).unwrap(),
        )
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        ctx.set_visuals(egui::Visuals::dark());

        let enabled = true;

        egui::SidePanel::right("right-panel")
            .default_width(15.0)
            .min_width(0f32)
            .resizable(false)
            .show(ctx, |ui| {
                ui.set_enabled(enabled);
                ui.with_layout(
                    egui::Layout::bottom_up(
                        egui::Align::TOP,
                    ),
                    |ui| {
                        if ui.button("V").clicked() {
                            self.glbb.pos.y -= 100.0;
                        }

                        if ui.button("V\nV").clicked() {
                            self.glbb.vertical.fall();
                        }

                        let max = self.glbb.pos_y_max();

                        ui.add(
                            slider::Slider::new(
                                &mut self.glbb.pos.y,
                                0f32..=max,
                            )
                            .vertical(),
                        );
                    },
                );
            });

        egui::TopBottomPanel::bottom("bottom-panel").show(
            ctx,
            |ui| {
                let max = self.glbb.pos_x_max();
                ui.add_enabled_ui(enabled, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 20.0],
                        slider::Slider::new(
                            &mut self.glbb.pos.x,
                            0f32..=max,
                        ),
                    );
                });

                ui.horizontal(|ui| {
                    let height = 20.0;
                    let width = ui.available_width()
                        - (ui.spacing().item_spacing.x
                            * 5.0);
                    let enabled =
                        !self.glbb.horizontal.is_play();

                    ui.add_enabled_ui(enabled, |ui| {
                        ui.set_enabled(enabled);

                        if ui
                            .add_sized(
                                [width * 0.1, height],
                                egui::Button::new("<<"),
                            )
                            .clicked()
                        {
                            self.glbb
                                .horizontal
                                .play_left();
                        }

                        if ui
                            .add_sized(
                                [width * 0.1, height],
                                egui::Button::new("<"),
                            )
                            .clicked()
                        {
                            self.glbb.pos.x -= 100.0;
                        }

                        ui.add_sized(
                            [width * 0.2, height],
                            egui::DragValue::new(
                                &mut self
                                    .glbb
                                    .horizontal
                                    .velocity,
                            )
                            .prefix("velocity: ")
                            .suffix(" m/s")
                            .clamp_range(0f32..=f32::MAX),
                        );
                    });

                    if ui
                        .add_sized(
                            [width * 0.2, height],
                            egui::Button::new("| |"),
                        )
                        .clicked()
                    {
                        self.glbb.horizontal.stop();
                        self.glbb.vertical.stop();
                    }

                    ui.add_enabled_ui(enabled, |ui| {
                        ui.add_sized(
                            [width * 0.2, height],
                            egui::DragValue::new(
                                &mut self
                                    .glbb
                                    .horizontal
                                    .acceleration,
                            )
                            .prefix("acceleration: ")
                            .suffix(" m/s²")
                            .clamp_range(1f32..=f32::MAX),
                        );

                        if ui
                            .add_sized(
                                [width * 0.1, height],
                                egui::Button::new(">"),
                            )
                            .clicked()
                        {
                            self.glbb.pos.x += 100.0;
                        }

                        if ui
                            .add_sized(
                                [width * 0.1, height],
                                egui::Button::new(">>"),
                            )
                            .clicked()
                        {
                            self.glbb
                                .horizontal
                                .play_right();
                        }
                    });
                })
            },
        );

        let response = egui::CentralPanel::default().show(ctx, |ui| {
            // make GLBBWidget expand to minimum available size.
            ui.vertical_centered_justified(|ui| {
                ui.with_layout(
                    egui::Layout::left_to_right().with_cross_justify(true),
                    |ui| {
                        let response = GLBBWidget::new(&mut self.glbb)
                            .show(ui)
                            .interact(egui::Sense::click_and_drag());

                        if enabled && !self.glbb.is_play() {
                            let dragged_by_secondary =
                                response.dragged_by(egui::PointerButton::Secondary);
                            let drag_released = response.drag_released();

                            {
                                let mut memory = ui.memory();
                                let is_dragged = memory.data.get_temp_mut_or_default::<bool>(
                                    egui::Id::new("glbb-dragged-with-right-button"),
                                );

                                if *is_dragged && drag_released {
                                    self.glbb.vertical.fall();
                                }

                                *is_dragged = dragged_by_secondary;
                            }
                            if let Some(pos) = response.interact_pointer_pos() {
                                self.glbb.pos = self.glbb.pos_from_screen(response.rect, pos);
                            }
                        }
                    },
                );
            })
        });

        self.size = response.response.rect.size();
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "glbb",
        native_options,
        Box::new(|it| Box::new(App::new(it))),
    );
}

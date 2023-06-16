use eframe::egui;
use egui::Color32;
use egui::Stroke;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    cx: f32,
    cy: f32,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            cx: 100.0,
            cy: 100.0,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("App", |ui| {
                    if ui.button("Save").clicked() {
                        println!("Save button - print")
                    }
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.label("This is a ui.label");

            if ui.button("Quit").clicked() {
                frame.close()
            }

            let painter = ui.painter();

            if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                painter.circle(
                    pos,
                    50.0,
                    Color32::TRANSPARENT,
                    Stroke {
                        width: 2.0,
                        color: Color32::from_rgb(255, 255, 255),
                    },
                );
            }
        });

        // ctx.request_repaint();
    }
}

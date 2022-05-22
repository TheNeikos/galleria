#[cfg(not(tarpaulin_include))]
fn main() {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Galleria",
        native_options,
        Box::new(|cc| Box::new(GalleriaApp::new(cc))),
    );
}

#[derive(Debug, Default)]
struct GalleriaApp {}

impl GalleriaApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GalleriaApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

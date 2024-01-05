use eframe::egui;
use egui_hooks::UseHookExt as _;

fn main() {
    eframe::run_native(
        "egui_hooks-example-use_persistent_state",
        Default::default(),
        Box::new(|_| Box::new(MyApp)),
    )
    .unwrap();
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let count = ui.use_persistent_state(0usize, ());
            ui.label(format!("Count: {}", count));
            if ui.button("Increment").clicked() {
                count.set_next(*count + 1);
            }
        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(1500)
    }
}

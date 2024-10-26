use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::job::Job;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PrinterApp {
    pub port: u16,
    #[serde(skip)] // This how you opt-out of serialization of a field
    pub rx: Option<Receiver<Message>>,
}

impl PrinterApp {
    pub fn new(rx: Receiver<Message>) -> Self {
        Self {
            rx: Some(rx),
            ..Default::default()
        }
    }
}

impl Default for PrinterApp {
    fn default() -> Self {
        Self {
            port: 6981,
            rx: None,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    WaitSocket,
    QRCode(String),
    CheckJob(Job),
    Success,
}

pub struct AppWrapper {
    pub app: Arc<Mutex<PrinterApp>>,
}

impl AppWrapper {
    pub fn new(app: Arc<Mutex<PrinterApp>>) -> Self {
        Self { app }
    }
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.lock().unwrap().update(ctx, frame);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("save");
        self.app.lock().unwrap().save(storage);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // TODO: Fix it. Exit in a more graceful way.
        sleep(Duration::from_micros(800));
        // Signal the other threads to stop
        std::process::exit(0);
    }
}

impl eframe::App for PrinterApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("save app");
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(rx) = &mut self.rx {
            if let Ok(message) = rx.try_recv() {
                // 处理消息
                println!("Message {:?}", message);
            }
        }
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Transparent").clicked() {
                        // Change the view to invisible.
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Settings");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });
            ui.label("Port");

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.port).range(0..=65535));
                if ui.button("Default").clicked() {
                    self.port = 6981;
                }
            });

            ui.vertical_centered_justified(|ui| {
                ui.heading("Scan the QR Code");
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

// #[tokio::main]
// async fn main() -> eframe::Result {
//     let native_options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_inner_size([400.0, 300.0])
//             .with_min_inner_size([300.0, 220.0]),
//         ..Default::default()
//     };

//     // Create a global App
//     let (tx, rx) = mpsc::channel(100);
//     // tokio::spawn(async move {
//     //     tx.send(Message::QRCode()).await.unwrap();
//     // });

//     let app = Arc::new(Mutex::new(PrinterApp::new(rx)));
//     let app_wrapper = Box::new(AppWrapper::new(Arc::clone(&app)));
//     let app_clone = Arc::clone(&app);

//     tokio::spawn(async move {
//         loop {
//             sleep(Duration::from_secs(1));
//             let mut app = app_clone.lock().unwrap();
//             (*app).port += 1;
//         }
//     });

//     eframe::run_native(
//         "RupmPrinter",
//         native_options,
//         Box::new(move |cc| {
//             // Change the app in app_wrapper
//             if let Some(storage) = cc.storage {
//                 if let Some(mut app) = eframe::get_value::<PrinterApp>(storage, eframe::APP_KEY) {
//                     let mut app_wrapper_locked = app_wrapper.app.lock().unwrap();
//                     app.rx = app_wrapper_locked.rx.take();
//                     *app_wrapper_locked = app;
//                 }
//             }

//             Ok(app_wrapper)
//         }),
//     )
// }

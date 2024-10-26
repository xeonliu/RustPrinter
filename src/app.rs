use eframe::App;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::Sender;

use crate::job::Job;
use egui::ColorImage;
use egui::Label;
use image::{GenericImage, Luma};
use qrcode::QrCode;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum Message {
    // Server & Backend
    WaitSocket,
    BytesReceived(usize),
    QRCode(String),
    CheckJob(Job),
    Success,
}

#[derive(Debug)]
pub enum AppMessage {
    // App
    Port(u16),
    Confirm(bool),
}

#[derive(Debug, Default)]
enum State {
    #[default]
    Waiting,
    Logging(String),
    Confirm(Job),
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PrinterApp {
    pub port: u16,
    #[serde(skip)] // This how you opt-out of serialization of a field
    pub rx: Option<Receiver<Message>>,
    #[serde(skip)]
    pub tx2: Option<Sender<AppMessage>>,
    #[serde(skip)]
    state: State,
}

impl PrinterApp {
    pub fn new(rx: Receiver<Message>, tx2: Sender<AppMessage>) -> Self {
        Self {
            rx: Some(rx),
            tx2: Some(tx2),
            ..Default::default()
        }
    }
}

impl Default for PrinterApp {
    fn default() -> Self {
        Self {
            port: 6981,
            rx: None,
            tx2: None,
            state: State::Waiting,
        }
    }
}

impl eframe::App for PrinterApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("save app");
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        sleep(Duration::from_millis(800));
        std::process::exit(0);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Settings");

            ui.label("Port");

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.port).range(0..=65535));
                if ui.button("Default").clicked() {
                    self.port = 6981;
                }
            });

            if let Some(rx) = &mut self.rx {
                if let Ok(message) = rx.try_recv() {
                    // 处理消息
                    println!("Message {:?}", message);
                    match message {
                        Message::WaitSocket => self.state = State::Waiting,
                        Message::QRCode(link) => self.state = State::Logging(link),
                        Message::CheckJob(job) => self.state = State::Confirm(job),
                        Message::Success => self.state = State::Waiting,
                        Message::BytesReceived(num) => {
                            ctx.request_repaint();
                            ui.label(format!("Received bytes: {}", num));
                        }
                    }
                }
            }

            match &self.state {
                State::Waiting => {
                    ui.label("Waiting for TCP Connection...");
                }
                State::Logging(link) => {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Scan the QR Code");
                        // Encode some data into bits.
                        let code = QrCode::new(link).unwrap();
                        let image = code.render::<Luma<u8>>().max_dimensions(200, 200).build();
                        let image_buffer = image.to_vec();
                        let size = [image.width() as usize, image.height() as usize];
                        let color_image = ColorImage::from_gray(size, &image_buffer);
                        // TODO: Only Once Per Image??
                        let texture =
                            ui.ctx()
                                .load_texture("qrcode", color_image, Default::default());
                        ui.add(egui::Image::from_texture(&texture));
                    });
                    ui.separator();
                    ui.label("Or Copy the link and open it in WeChat");
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut link.as_str())); // Wierd Fix
                        if ui.button("Copy").clicked() {
                            ui.output_mut(|o| o.copied_text = link.to_string());
                            ui.label("Copied");
                        };
                    });
                }
                State::Confirm(job) => {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Job Detail");
                        ui.label(serde_yaml::to_string(&job).unwrap());
                    });

                    ui.horizontal_centered(|ui| {
                        let port = self.port;
                        if ui.button("Confirm").clicked() {
                            if let Some(tx) = &self.tx2 {
                                let tx = tx.clone();
                                tokio::spawn(async move {
                                    tx.send(AppMessage::Confirm(true)).await.unwrap();
                                    tx.send(AppMessage::Port(port)).await.unwrap();
                                });
                            }
                        }

                        if ui.button("Cancel").clicked() {
                            // Tell Main Program to proceed
                            if let Some(tx) = &self.tx2 {
                                let tx = tx.clone();
                                tokio::spawn(async move {
                                    tx.send(AppMessage::Confirm(false)).await.unwrap();
                                    tx.send(AppMessage::Port(port)).await.unwrap();
                                });
                            }
                        }
                    });
                }
                _ => {}
            }

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

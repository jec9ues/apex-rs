mod egui_overlay;
pub mod mem;
pub mod constants;
pub mod function;
pub mod math;
pub mod data;
pub mod cache;
pub mod aimbot;
pub mod config;
pub mod menu;


use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;
use std::sync::Once;
use egui_backend::{WindowBackend};
use egui_overlay::EguiOverlay;


use egui_render_three_d::ThreeDBackend as DefaultGfxBackend;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::*;
use egui_backend::egui::{Color32, Id, LayerId, Order, Painter, Pos2, Rect, Shape, Stroke, Key, CollapsingHeader, RichText};

use egui_backend::egui::plot::{Line, Plot, PlotPoints};

use log4rs;
use log::{debug, info};
use memprocfs::*;
use crate::cache::Data;
use crate::function::*;
use crate::math::world_to_screen;
use crate::mem::*;
use rand::Rng;
use crate::aimbot::main_aimbot;
use crate::config::{Config, MenuConfig};
use crate::menu::{edit_aimbot_config, edit_esp_config, edit_glow_config, edit_screen_size};


fn setup_custom_fonts(ctx: &egui_backend::egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui_backend::egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui_backend::egui::FontData::from_static(include_bytes!(
            "zh_CN.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui_backend::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui_backend::egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}


fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();



    let (config_sender, config_receiver) = bounded::<Config>(1);

    let (data_sender, data_receiver) = bounded::<Data>(1);

    let (restart_sender, restart_receiver) = bounded::<bool>(1);
    thread::spawn(move || {
        loop {

            main_mem(data_sender.clone(), config_receiver.clone(), restart_receiver.clone());
        }

    });


    egui_overlay::start(Menu {
        fps: FpsCounter::new(),
        data: Data::default(),
        data_recv: data_receiver,
        menu_config: MenuConfig::default(),
        config_sender,
        restart_sender
    });
}

// TODO: config channel
pub struct Menu {
    pub fps: FpsCounter,

    pub data: Data,
    pub data_recv: Receiver<Data>,

    pub menu_config: MenuConfig,
    pub config_sender: Sender<Config>,
    
    pub restart_sender: Sender<bool>
}

impl EguiOverlay for Menu {
    fn gui_run(
        &mut self,
        egui_context: &egui_backend::egui::Context,
        _default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        static ONCE: Once = Once::new();

        ONCE.call_once( || {
            setup_custom_fonts(egui_context);
            glfw_backend.set_window_position([0., 0.]);
            glfw_backend.set_window_size([2570.0f32, 1440.0f32]);
        });


        // just some controls to show how you can use glfw_backend
        // let mut da: Data = Data::default();
        let overlay = Painter::new(egui_context.clone(), LayerId::new(Order::TOP, Id::new("overlay")), Rect::EVERYTHING);


        match self.data_recv.try_recv() {
            Ok(data) => {
                // println!("Received message from thread {:?}", data);
                self.data = data;
            }
            Err(_) => {}
        };
        // println!("most far distance -> {}", self.data.get_near_pointer());
        // self.data.draw_bones_width(overlay.clone());
        if self.menu_config.config.esp.enable {
            self.data.cache_data.target.target_line(overlay.clone(), self.data.config.screen.center);
            if self.data.cache_data.target.status.visible() {
                self.data.cache_data.target.bone_esp(overlay.clone(), 999.0, Color32::GREEN);
            } else {
                self.data.cache_data.target.bone_esp(overlay.clone(), 999.0, Color32::RED);
            }

            // overlay.circle_stroke(self.data.cache_data.target.get_nearest_bone(self.menu_config.config.screen.center).position_2d, self.menu_config.config.aim.aim_assist.zone, Stroke::new(3.0, Color32::RED));
            for player in &self.data.cache_data.players {
                // player.1.box_esp(overlay.clone());
                if player.1.distance < self.menu_config.config.esp.distance {
                    player.1.position_esp(overlay.clone());
                }

                // player.1.target_line(overlay.clone());
            }
        }




        egui_backend::egui::Window::new("Debug").vscroll(true).show(egui_context, |ui| {
            ui.set_width(450.0);

            glfw_backend.window.set_decorated(false);
            self.fps.update();
            ui.label(format!("current fps: {}", self.fps.fps()));
            ui.label(format!("cursor pos x: {}", glfw_backend.cursor_pos[0]));
            ui.label(format!("cursor pos y: {}", glfw_backend.cursor_pos[1]));
            ui.label(format!(
                "passthrough: {}",
                glfw_backend.get_passthrough().unwrap()
            ));

            self.data.dbg_view(ui);
            // ui.label(format!("{:?}", self.data.cache_data.target));

        });

        egui_backend::egui::Window::new("Menu").show(egui_context, |ui| {
            // ui.set_width(300.0);

            ui.horizontal(|ui| {
                if ui.button("restart dma connect").clicked() {
                    self.restart_sender.send(true).expect("restart send failed");
                    self.data = Data::default();
                }

                if ui.button("save config").clicked() {
                    self.menu_config.save();
                }

                if ui.button("load config").clicked() {
                    self.menu_config.load();
                }
            });

            edit_screen_size(&mut self.menu_config.config.screen, ui);
            ui.vertical( |ui| {

                edit_glow_config(&mut self.menu_config.config.glow, ui);

                edit_esp_config(&mut self.menu_config.config.esp, ui);

                edit_aimbot_config(&mut self.menu_config.config.aim, ui);
                ui.label(format!("config: {:?}", self.menu_config.config));

            });

        });

        egui_backend::egui::Window::new("Curve Preview").show(egui_context, |ui| {
            CollapsingHeader::new(RichText::new("yaw curve preview"))
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical( |ui| {
                        let sin: PlotPoints = (1..1000).step_by(1).map(|i| {
                            let x = calculate_delta_smooth(i as f32, self.menu_config.config.aim.aim_assist.yaw_smooth, self.menu_config.config.aim.aim_assist.yaw_curve_factor) as f64;
                            [x , x.powi(2)]
                        }).collect();
                        let line = Line::new(sin);
                        Plot::new("yaw curve preview")
                            .view_aspect(1.0)
                            .show_axes([true, true])
                            .show(ui, |plot_ui| plot_ui.line(line));
                    });
                });


            CollapsingHeader::new(RichText::new("pitch curve preview"))
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical( |ui| {
                        let sin: PlotPoints = (1..1000).step_by(1).map(|i| {
                            let x = calculate_delta_smooth(i as f32, self.menu_config.config.aim.aim_assist.pitch_smooth, self.menu_config.config.aim.aim_assist.pitch_curve_factor) as f64;
                            [x , x.powi(2)]
                        }).collect();
                        let line = Line::new(sin);
                        Plot::new("pitch curve preview")
                            .view_aspect(1.0)
                            .show_axes([true, true])
                            .show(ui, |plot_ui| plot_ui.line(line));
                    });
                });

        });
        match self.config_sender.try_send(self.menu_config.config) {
            Ok(_) => {}
            Err(_) => {}
        }

        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }

        egui_context.request_repaint();
    }
}






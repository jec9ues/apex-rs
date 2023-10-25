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
pub mod network;
pub mod kmbox_bpro;


use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;
use std::sync::Once;
use egui_backend::{WindowBackend};
use egui_overlay::EguiOverlay;


use egui_render_three_d::ThreeDBackend as DefaultGfxBackend;
use std::{env, thread};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::*;
use egui_backend::egui::{Color32, Id, LayerId, Order, Painter, Pos2, Rect, Shape, Stroke, Key, CollapsingHeader, RichText, ColorImage, TextureHandle, Vec2};

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
use crate::menu::{edit_aimbot_config, edit_esp_config, edit_glow_config, edit_screen_size, edit_world_config};
use crate::network::verify_key;



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

        // if verify_key().unwrap() != "valid" {
        //     std::process::exit(1);
        // }


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
        texture: None,
        menu_config: MenuConfig::default(),
        config_sender,
        restart_sender,
    });
}

// TODO: config channel
pub struct Menu {
    pub fps: FpsCounter,

    pub data: Data,
    pub data_recv: Receiver<Data>,
    pub texture: Option<TextureHandle>,
    pub menu_config: MenuConfig,
    pub config_sender: Sender<Config>,

    pub restart_sender: Sender<bool>,

}

impl EguiOverlay for Menu {
    fn gui_run(
        &mut self,
        egui_context: &egui_backend::egui::Context,
        _default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
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
            self.data.cache_data.target.target_line(overlay.clone(), self.data.config.screen.center());
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
            ui.label(format!("{:?}", self.data.cache_data.local_player.position));
            if ui.button("📋").clicked() {
                ui.output_mut(|o| o.copied_text = format!("{:?}", self.data.cache_data.local_player.position));
            }
        });

        egui_backend::egui::Window::new("Menu").show(egui_context, |ui| {
            // ui.set_width(300.0);

            ui.horizontal(|ui| {
                if ui.button("restart DMA connect").clicked() {
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
            ui.vertical(|ui| {
                edit_glow_config(&mut self.menu_config.config.glow, ui);

                edit_esp_config(&mut self.menu_config.config.esp, ui);

                edit_aimbot_config(&mut self.menu_config.config.aim, ui);
                edit_world_config(&mut self.menu_config.config.world, ui);
                ui.label(format!("config: {:?}", self.menu_config.config));
            });
        });

        egui_backend::egui::Window::new("Curve Preview").show(egui_context, |ui| {
            CollapsingHeader::new(RichText::new("yaw curve preview"))
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        let sin: PlotPoints = (1..1000).step_by(1).map(|i| {
                            let x = calculate_delta_smooth(i as f32, self.menu_config.config.aim.aim_assist.yaw_smooth, self.menu_config.config.aim.aim_assist.yaw_curve_factor) as f64;
                            [x, x.powi(2)]
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
                    ui.vertical(|ui| {
                        let sin: PlotPoints = (1..1000).step_by(1).map(|i| {
                            let x = calculate_delta_smooth(i as f32, self.menu_config.config.aim.aim_assist.pitch_smooth, self.menu_config.config.aim.aim_assist.pitch_curve_factor) as f64;
                            [x, x.powi(2)]
                        }).collect();
                        let line = Line::new(sin);
                        Plot::new("pitch curve preview")
                            .view_aspect(1.0)
                            .show_axes([true, true])
                            .show(ui, |plot_ui| plot_ui.line(line));
                    });
                });
        });

/*        egui_backend::egui::Window::new("Map").title_bar(false).show(egui_context, |ui| {
            let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
                // Load the texture only once.
                let path: PathBuf = env::current_dir().unwrap();
                ui.ctx().load_texture(
                    "Olympus_MU2_REV1.webp",
                    load_image_from_path(&path.join("Olympus_MU1.webp")).unwrap(),
                    Default::default()
                )
            });
            // ui.image(texture, Vec2::new(1024.0, 1024.0));
            let map_ptr = Painter::new(egui_context.clone(), LayerId::new(Order::TOP, Id::new("map_ptr")), Rect::EVERYTHING);
            let player_pos = convert_coordinates(
                self.data.cache_data.local_player.position.x,
                self.data.cache_data.local_player.position.y,
                self.data.config.world.world_min_x,
                self.data.config.world.world_min_y,
            );
            for player in &self.data.cache_data.players {
                // player.1.box_esp(overlay.clone());
                player.1.map_esp(overlay.clone(), ui.min_rect().min, [self.menu_config.config.world.world_min_x, self.menu_config.config.world.world_min_y]);

            }
            //Pos3 { x: -830.64374, y: 14927.562, z: -5838.49 }
            //Pos3 { x: -830.6441, y: 14927.562, z: -5838.49 }

            // Right Bottom Pos3 { x: 21686.338, y: -26536.438, z: 3607.3196 } king
            // left top Pos3 { x: -29279.21, y: 25624.809, z: 3310.6152 } king
            // left top Pos3 { x: -30171.703, y: 32398.465, z: -3045.3596 } edge
            // right bottom Pos3 { x: 22957.547, y: -43407.934, z: -1930.706 } edge
            map_ptr.circle(Pos2 { x: ui.min_rect().min.x + player_pos.0, y: ui.min_rect().min.y + player_pos.1},
                           1.0, Color32::TRANSPARENT, Stroke::new(5.0, Color32::RED));
            map_ptr.circle(ui.min_rect().min, 1.0, Color32::TRANSPARENT, Stroke::new(5.0, Color32::RED));
        });*/
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

pub fn load_image_from_path(path: &std::path::Path) -> Result<ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
// Pos3 { x: -23447.604, y: 37496.742, z: -6781.5986 }
// Pos3 { x: -17642.447, y: 39367.54, z: -6782.021 }
pub fn convert_coordinates( player_x: f32, player_y: f32, map_width: f32, map_height: f32) -> (f32, f32) {
    // 定义地图坐标范围
    let map_left_top = Pos2 { x: -52024.0, y: 48025.0 };
    let map_right_bottom = Pos2 { x: 22957.547, y: -43407.934 };

    // 计算地图的宽度和高度
    // let map_width = map_right_bottom.x - map_left_top.x;
    // let map_height = map_left_top.y - map_right_bottom.y;

    // 计算地图坐标
    let map_x = ((player_x - map_left_top.x) / map_width) * 1024.0;
    let map_y = 1024.0 - ((player_y - map_right_bottom.y) / map_height) * 1024.0;

    (map_x, map_y)
}


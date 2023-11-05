mod egui_overlay;
pub mod function;
pub mod math;
pub mod data;
pub mod cache;
pub mod config;
pub mod menu;
pub mod network;


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
use crate::math::{angles_to_screen, calculate_desired_yaw, flip_yaw, flip_yaw_if_needed, world_to_screen};
use rand::Rng;
use crate::config::{Config, MenuConfig};
use crate::data::Pos3;
use crate::menu::{edit_aimbot_config, edit_esp_config, edit_glow_config, edit_screen_size, edit_world_config};
use crate::network::recv_main;


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
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();




    let (config_sender, config_receiver) = bounded::<Config>(1);

    let (data_sender, data_receiver) = bounded::<Data>(1);

    let network = thread::spawn( move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(
            recv_main(
                data_sender.clone(),
                config_receiver.clone()
            )
        );
    });



    egui_overlay::start(Menu {
        fps: FpsCounter::new(),
        data: Data::default(),
        data_receiver,
        config_sender,
        texture: None,
        menu_config: MenuConfig::default(),

    });

    network.join().unwrap()
}

// TODO: config channel
pub struct Menu {
    pub fps: FpsCounter,

    pub data: Data,
    pub data_receiver: Receiver<Data>,
    pub texture: Option<TextureHandle>,
    pub menu_config: MenuConfig,
    pub config_sender: Sender<Config>,


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
        // println!("overlay");
        match self.config_sender.try_send(self.menu_config.config) {
            Ok(_) => {}
            Err(_) => {}
        }

        match self.data_receiver.try_recv() {
            Ok(mut data) => {
                // println!("Received message from thread {:?}", data);
                self.data = data;
            }
            Err(_) => {}
        };

        fn calculate_grenade_pitch(distance: f32) -> f32 {
            let initial_value = -84.0;
            let initial_second_data = 10.0;
            let final_value = -81.0;
            let final_second_data = 15.0;

            let growth_rate = (final_value - initial_value) / (final_second_data - initial_second_data);

            let first_data = initial_value + (distance - initial_second_data) * growth_rate;

            // 限制第一个数据的范围在 -89 到 89 之间
            if first_data < -89.0 {
                return -89.0;
            } else if first_data > 89.0 {
                return 89.0;
            }

            first_data
        }

        for (_, player) in &self.data.cache_data.players {
            let grenade_yaw = calculate_desired_yaw(self.data.cache_data.local_player.camera_position, player.position);
            let grenade_pitch = calculate_grenade_pitch(player.distance_2d);
            // println!("pitch -> {grenade_pitch} yaw -> {grenade_yaw}");
            let np = angles_to_screen(
                &self.data.cache_data.local_player,
                Pos3 {/*pitch*/ x: -grenade_pitch,
                    /*yaw*/   y: flip_yaw(grenade_yaw) , z: 0.0},
                self.data.config.screen.size,
            );
            overlay.circle(
                np,
                5.0,
                Color32::TRANSPARENT,
                Stroke::new(2.0, Color32::GREEN
                ));
        }
        // println!("{:?}", self.data.cache_data.local_player.camera_position);
        // println!("{:?}", np);

        // overlay.line_segment([self.data.cache_data.local_player.hitbox.head.position_2d,
        // cp], Stroke::new(2.0, Color32::BLUE));
        // println!("center func -> {:?}", self.data.config.screen);
        // self.data.draw_bones_width(overlay.clone());
        if self.menu_config.config.esp.enable {
            self.data.cache_data.target.target_line(overlay.clone(), self.menu_config.config.screen.center());
            overlay.circle(
                self.menu_config.config.screen.center(),
                5.0,
                Color32::TRANSPARENT,
                Stroke::new(2.0, Color32::GREEN
                ));
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
            ui.label(format!("pitch -> {:?}", self.data.cache_data.local_player.pitch));
            ui.label(format!("yaw -> {:?}", self.data.cache_data.local_player.yaw));
            if ui.button("📋").clicked() {
                ui.output_mut(|o| o.copied_text = format!("{:?}", self.data.cache_data.local_player.position));
            }
        });

        egui_backend::egui::Window::new("Menu").show(egui_context, |ui| {
            // ui.set_width(300.0);

            ui.horizontal(|ui| {
                if ui.button("reset data").clicked() {
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

/*function getDataByLevelName(levelName: string) {
switch (levelName) {
case 'mp_rr_canyonlands_mu3':
return {x: -37541, y: 43886};
case 'mp_rr_desertlands_mu3':
return {x: -45056, y: 45055};
case 'mp_rr_olympus_mu2':
return {x: -52024, y: 48025};
case 'mp_rr_tropic_island_mu1':
return {x: -50606, y: 52139};
default:
return;
}
}*/


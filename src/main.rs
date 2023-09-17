mod egui_overlay;
pub mod mem;
pub mod constants;
pub mod function;
pub mod math;
pub mod data;


use std::sync::Once;
use egui_backend::{WindowBackend};
use egui_overlay::EguiOverlay;


use egui_render_three_d::ThreeDBackend as DefaultGfxBackend;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::*;
use egui_backend::egui::{Color32, Id, LayerId, Order, Painter, Pos2, Rect, Shape, Stroke, Key};

use log4rs;
use log::debug;
use memprocfs::*;
use crate::function::*;
use crate::mem::main_mem;


fn box_2d(ptr: Painter, loc: Pos2, width: f32, color: Color32) {
    // let draw = Rect::from_min_size(loc, Vec2::new(10.0, 20.0));
    ptr.circle(loc, 3.0, Color32::TRANSPARENT, Stroke::new(width, color))
    // ptr.rect(draw, Rounding::same(0.0),Color32::TRANSPARENT ,Stroke::new(width, color));

}






fn main() {
    // TODO: loop memory to data struct; loop config(from egui) to run cheat; loop script to manage config(for egui);
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let (sender, receiver) = unbounded::<Vec<Pos2>>();




    let cheat = thread::spawn(move || {
        main_mem(sender);
        }
    );


    egui_overlay::start(Menu {data: Vec::new(), frame: 0, menu_on: true, last_frame_time: Instant::now(), fps: 0.0 , da: receiver});
    cheat.join().unwrap()
}

pub struct Menu {
    pub frame: u64,
    pub menu_on: bool,
    pub last_frame_time: Instant,
    pub fps: f32,
    pub da: Receiver<Vec<Pos2>>,
    pub data: Vec<Pos2>,
}

impl EguiOverlay for Menu {
    fn gui_run(
        &mut self,
        egui_context: &egui_backend::egui::Context,
        _default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {

        static ONCE: Once = Once::new();

        // 使用 Once 标志来判断代码是否应该运行
        ONCE.call_once(|| {
            glfw_backend.set_window_position([0.,0.]);
            glfw_backend.set_window_size([2570.0f32,1440.0f32]);
            // println!("This code runs only once.");
        });

        // 注册一个键盘事件回调函数
        egui_context.input(|i| {
            if i.key_pressed(Key::Insert) {
                println!("pressed");
                self.menu_on = !self.menu_on;
            }
        });

        // 计算上一帧到当前帧的时间间隔
        let now = Instant::now();
        let delta_time = now - self.last_frame_time;
        self.last_frame_time = now;

        // 计算每秒帧数
        self.fps = 1.0 / delta_time.as_secs_f32();
        // just some controls to show how you can use glfw_backend
        // let mut da: Data = Data::default();
        let overlay = Painter::new(egui_context.clone(), LayerId::new(Order::TOP, Id::new("overlay")),Rect::EVERYTHING);


        match self.da.try_recv() {
            Ok(data) => {
                // println!("Received message from thread {:?}", data);
                self.data = data;
            }
            Err(_) => { }
        };


        for i in &self.data {
            box_2d(overlay.clone(), Pos2::new(i.x, i.y), 2.0, Color32::RED);
        };
        if self.menu_on {
            egui_backend::egui::Window::new("controls").show(egui_context, |ui| {
/*                let sin: PlotPoints = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                }).collect();
                let line = Line::new(sin);
                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));*/

                ui.set_width(300.0);
                self.frame += 1;
                glfw_backend.window.set_decorated(false);
                ui.label(format!("current frame number: {}", self.frame));
                ui.label(format!("current fps: {}", self.fps as u32));


                ui.label(format!(
                    "pixels_per_virtual_unit: {}",
                    glfw_backend.physical_pixels_per_virtual_unit
                ));
                ui.label(format!("window scale: {}", glfw_backend.scale));
                ui.label(format!("cursor pos x: {}", glfw_backend.cursor_pos[0]));
                ui.label(format!("cursor pos y: {}", glfw_backend.cursor_pos[1]));

                ui.label(format!(
                    "passthrough: {}",
                    glfw_backend.get_passthrough().unwrap()
                ));
                // how to change size.
                // WARNING: don't use drag value, because window size changing while dragging ui messes things up.
            });

            egui_backend::egui::Window::new("Menu").show(egui_context, |ui| {
                ui.set_width(300.0);
                glfw_backend.window.set_decorated(false);
            });
        }

        // here you decide if you want to be passthrough or not.
        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }
        egui_context.request_repaint();
    }
}



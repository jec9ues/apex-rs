use egui_backend::egui;
use egui_backend::egui::{CollapsingHeader, Color32, ComboBox, DragValue, RichText, Slider, Ui, Widget, WidgetText};
use crate::config::{AimConfig, EspConfig, GlowConfig, ScreenConfig};


use crate::data::*;
pub fn dbg_status(status: &Status, ui: &mut Ui) {
    CollapsingHeader::new(format!("{}'s status", status.name))
        .default_open(false)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("dead -> {}", status.dead));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.dead.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("knocked -> {}", status.knocked));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.knocked.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("health -> {}", status.health));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.health.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("max_health -> {}", status.max_health));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.max_health.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("shield -> {}", status.shield));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.shield.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("max_shield -> {}", status.max_shield));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.max_shield.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("helmet_type -> {}", status.helmet_type));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.helmet_type.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("armor_type -> {}", status.armor_type));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.armor_type.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("last_visible_time -> {}", status.last_visible_time));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.last_visible_time.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("previous_last_visible_time -> {}", status.previous_last_visible_time));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.previous_last_visible_time.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("last_crosshair_target_time -> {}", status.last_crosshair_target_time));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.last_crosshair_target_time.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("previous_last_crosshair_target_time -> {}", status.previous_last_crosshair_target_time));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.previous_last_crosshair_target_time.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("skin -> {}", status.skin));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.skin.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("character -> {:?}", status.character));
                    if ui.button("📋").clicked() {
                        // 这里根据你的数据结构来输出角色的值
                        ui.output_mut(|o| o.copied_text = format!("{:?}", status.character));
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("team -> {}", status.team));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.team.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("team_index -> {}", status.team_index));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.team_index.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("name -> {}", status.name));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.name.to_string());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("platform_id -> {}", status.platform_id));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.platform_id.to_string());
                    }
                });
            });
        });
}
pub fn dbg_player(player: &Player, ui: &mut Ui) {
    ui.group( |ui| {
        ui.vertical( |ui| {
            let mut title = RichText::new("Default");
            if player.status.dead > 0 {
                title = RichText::new(format!("{} -> {}", player.index, player.status.name)).color(Color32::RED).strikethrough();
            }  else {
                title = RichText::new(format!("{} -> {}", player.index, player.status.name)).color(Color32::GREEN);
            }
            CollapsingHeader::new(title)
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!("player pointer -> {:x}", player.pointer));
                    ui.label(format!("player distance -> {}", player.distance));
                    ui.label(format!("player distance -> {:?}", player.position));
                    dbg_status(&player.status, ui);
                });
        });
    });
}

pub fn edit_screen_size(screen_size: &mut ScreenConfig, ui: &mut Ui) {
    ui.group( |ui| {
        ui.vertical( |ui| {
            CollapsingHeader::new(RichText::new("Screen Size"))
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal( |ui| {
                        ui.label("screen width -> ");
                        ui.add(DragValue::new(&mut screen_size.size[0]).speed(10.0));
                    });
                    ui.horizontal( |ui| {
                        ui.label("screen height -> ");
                        ui.add(DragValue::new(&mut screen_size.size[1]).speed(10.0));
                    });

                });
        });
    });
}

pub fn edit_aimbot_config(aim_config: &mut AimConfig, ui: &mut Ui) {
    ui.group( |ui| {

        // aim config
        ui.horizontal( |ui| {
            // aim assist config
            ui.vertical( |ui| {

                ui.checkbox(&mut aim_config.aim_assist.enable, "enable aim assist");

                ui.horizontal( |ui| {
                    ui.label("pitch smooth -> ");
                    ui.add(
                        Slider::new(&mut aim_config.aim_assist.pitch_smooth, 1.0..=100.0).step_by(1.0)
                    );
                });

                ui.horizontal( |ui| {
                    ui.label("yaw smooth -> ");
                    ui.add(
                        Slider::new(&mut aim_config.aim_assist.yaw_smooth, 1.0..=100.0).step_by(1.0)
                    );
                });

                combobox_key(&mut aim_config.aim_assist.key, ui, "aim assist bind key");

            });

            ui.vertical( |ui| {

                ui.checkbox(&mut aim_config.trigger_bot.enable, "enable trigger bot");

                combobox_key(&mut aim_config.trigger_bot.key, ui, "trigger bot bind key");
            });

            ui.vertical( |ui| {

            });

            ui.horizontal( |ui| {
                ui.label("distance -> ");
                ui.add(
                    Slider::new(&mut aim_config.distance, 1.0..=200.0).step_by(1.0)
                );
            });
        });
    });
}

pub fn edit_glow_config(glow_config: &mut GlowConfig, ui: &mut Ui) {
    ui.group( |ui| {
        // glow config
        ui.vertical( |ui| {
            ui.checkbox(&mut glow_config.player_glow.enable, "enable player glow");
        });
        ui.vertical( |ui| {
            ui.horizontal( |ui| {
                ui.label("delay -> ");
                ui.add(
                    Slider::new(&mut glow_config.player_glow.delay, 1..=1000).step_by(1.0)
                );
            });
        });
    });
}

pub fn edit_esp_config(esp_config: &mut EspConfig, ui: &mut Ui) {
    ui.group(|ui| {
        // esp config
        ui.vertical( |ui| {
            ui.checkbox(&mut esp_config.enable, "enable player esp");
        });
        ui.vertical( |ui| {
            ui.horizontal( |ui| {
                ui.label("distance -> ");
                ui.add(
                    Slider::new(&mut esp_config.distance, 1.0..=300.0).step_by(1.0)
                );
            });
        });

        ui.vertical( |ui| {
            ui.horizontal( |ui| {
                ui.label("delay -> ");
                ui.add(
                    Slider::new(&mut esp_config.delay, 1..=1000).step_by(1.0)
                );
            });
        });
    });
}


pub fn combobox_key(value: &mut InputSystem, ui: &mut Ui, text: &str) {
    ComboBox::from_label(text).selected_text(format!("{:?}", value)).show_ui(ui, |ui| {
        ui.selectable_value(value, InputSystem::MOUSE_LEFT, "left mouse");
        ui.selectable_value(value, InputSystem::MOUSE_RIGHT, "right mouse");
        ui.selectable_value(value, InputSystem::MOUSE_5, "front side mouse");
        ui.selectable_value(value, InputSystem::MOUSE_4, "back side mouse");
    });
}
use egui_backend::egui::*;
use crate::data::*;
pub fn status_ui(status: &Status, ui: &mut Ui) {
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
                    ui.label(format!("last_crosshair_target_time -> {}", status.last_crosshair_target_time));
                    if ui.button("📋").clicked() {
                        ui.output_mut(|o| o.copied_text = status.last_crosshair_target_time.to_string());
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
pub fn dbg_ui(player: &Player, ui: &mut Ui) {
    ui.group( |ui| {
        ui.vertical( |ui| {
            ui.label(format!("player index -> {}", player.index));
            ui.label(format!("player pointer -> {}", player.pointer));
            ui.label(format!("player distance -> {}", player.distance));
            // ui.label(format!("player rate -> {}", player.rate));

            status_ui(&player.status, ui);
            // pub index: u64,
            // pub pointer: u64,
            // pub bone_pointer: u64,
            // pub hitbox: Hitbox,
            // pub status: Status,
            // pub position: Pos3,
            // pub position_2d: Pos2,
            // pub distance: f32,
            // pub rate: f32,

        });
    });
}
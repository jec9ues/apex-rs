use egui_backend::egui::Pos2;
use crate::data::Pos3;

pub fn world_to_screen(matrix: [[f32; 4]; 4], vector: Pos3, screen_size: Pos2) -> Pos2 {
    let transformed = [
        vector.x * matrix[0][0] + vector.y * matrix[0][1] + vector.z * matrix[0][2] + matrix[0][3],
        vector.x * matrix[1][0] + vector.y * matrix[1][1] + vector.z * matrix[1][2] + matrix[1][3],
        vector.x * matrix[2][0] + vector.y * matrix[2][1] + vector.z * matrix[2][2] + matrix[2][3],
        vector.x * matrix[3][0] + vector.y * matrix[3][1] + vector.z * matrix[3][2] + matrix[3][3],
    ];

    if transformed[3] < 0.001 {
        return Pos2::new(0.0, 0.0);
    }

    let inv_w = 1.0 / transformed[3];
    let x = transformed[0] * inv_w;
    let y = transformed[1] * inv_w;

    let half_resolution = Pos2::new(screen_size.x / 2.0, screen_size.y / 2.0);

    Pos2::new(half_resolution.x + x * half_resolution.x, half_resolution.y - y * half_resolution.y)
}

pub fn distance3d(a: &Pos3, b: &Pos3) -> f32 {
    ((b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y) + (b.z - a.z) * (b.z - a.z)).sqrt() / 39.3701
}

pub fn distance2d(a: &Pos2, b: &Pos2) -> f32 {
    ((b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y)).sqrt()
}

pub fn calculate_desired_yaw(from: Pos3, to: Pos3) -> f32{
    let location_delta_x = to.x - from.x;
    let location_delta_y = to.y - from.y;
    let yaw_in_radians = location_delta_y.atan2(location_delta_x);
    let yaw_in_degrees = yaw_in_radians.to_degrees();
    yaw_in_degrees
}

pub fn calculate_desired_pitch(from: Pos3, to: Pos3) -> f32 {
    let location_delta_z = to.z - from.z;
    let distance_between_players = distance2d(
        &to.to_pos2(),
        &from.to_pos2(),
    );

    let pitch_in_radians = (-location_delta_z / distance_between_players).atan();
    let pitch_in_degrees = pitch_in_radians.to_degrees();

    pitch_in_degrees
}


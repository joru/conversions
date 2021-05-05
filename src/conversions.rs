#![allow(non_snake_case, non_upper_case_globals)]
use crate::tensors::{Mat, Vec3};

pub const toXYZ: Mat = Mat::new([
    Vec3::new(0.41239080, 0.35758434, 0.18048079),
    Vec3::new(0.21263901, 0.71516868, 0.07219232),
    Vec3::new(0.01933082, 0.11919478, 0.95053215),
]);
pub const fromXYZ: Mat = Mat::new([
    Vec3::new(3.24096994, -1.53738318, -0.49861076),
    Vec3::new(-0.96924364, 1.8759675, 0.04155506),
    Vec3::new(0.05563008, -0.20397696, 1.05697251),
]);

pub fn do_the_convertion(width: usize, height: usize, input: &[Vec3], output: &mut [u8]) -> () {
    for idx in 0..width * height {
        let rgb = convert_scene_linear_to_display_bytes(input[idx]);
        output[idx * 3 + 0] = rgb.0;
        output[idx * 3 + 1] = rgb.1;
        output[idx * 3 + 2] = rgb.2;
    }
}

pub fn convert_scene_linear_to_display_bytes(scene_linear_srgb: Vec3) -> (u8, u8, u8) {
    let display_linear_srgb = convert_scene_linear_to_display_linear(scene_linear_srgb);
    let red_byte = to_byte_clamp(sRGBcurve(display_linear_srgb.X()));
    let green_byte = to_byte_clamp(sRGBcurve(display_linear_srgb.Y()));
    let blue_byte = to_byte_clamp(sRGBcurve(display_linear_srgb.Z()));
    (red_byte, green_byte, blue_byte)
}

pub fn convert_scene_linear_to_display_linear(scene_linear_srgb: Vec3) -> Vec3 {
    let scene_linear = toXYZ * scene_linear_srgb;
    let lXYZ = scene_linear.X() + scene_linear.Y() + scene_linear.Z();
    let scene_linear_x = scene_linear.X() / lXYZ;
    let scene_linear_y = scene_linear.Y() / lXYZ;
    let white_point = toXYZ * Vec3::new(1.0, 1.0, 1.0);
    let WXYZ = white_point.X() + white_point.Y() + white_point.Z();
    let white_point_x = white_point.X() / WXYZ;
    let white_point_y = white_point.Y() / WXYZ;
    let scene_linear_x_offset = nan_to_zero(scene_linear_x - white_point_x);
    let scene_linear_y_offset = nan_to_zero(scene_linear_y - white_point_y);
    let _scene_linear_saturation = (scene_linear_x_offset * scene_linear_x_offset
        + scene_linear_y_offset * scene_linear_y_offset)
        .sqrt();
    let display_linear_Y = (scene_linear.Y() / (scene_linear.Y() + 1.0)).powf(1.0); //MAGIC
    let saturation_adjustment = (1.0 - display_linear_Y).powf(0.375); //MAGIC
    let display_linear_x_offset = scene_linear_x_offset * saturation_adjustment;
    let display_linear_y_offset = scene_linear_y_offset * saturation_adjustment;
    let display_linear_x = white_point_x + display_linear_x_offset;
    let display_linear_y = white_point_y + display_linear_y_offset;
    let dlXYZ = display_linear_Y / display_linear_y;
    let display_linear = Vec3::new(
        dlXYZ * display_linear_x,
        display_linear_Y,
        dlXYZ * (1.0 - display_linear_x - display_linear_y),
    );
    let display_linear_srgb = fromXYZ * display_linear;
    display_linear_srgb
}
fn nan_to_zero(x: f32) -> f32 {
    if x.is_nan() {
        0.0
    } else {
        x
    }
}
fn to_byte_clamp(f: f32) -> u8 {
    if f < 0.0 {
        return 0;
    }
    if f > 1.0 {
        return 255;
    }
    if f == 1.0 {
        return 255;
    }
    (f * 256.0).floor() as u8
}
fn sRGBcurve(dl: f32) -> f32 {
    if dl <= 0.0031308 {
        return dl * 12.92;
    }
    dl.powf(1.0 / 2.4) * 1.055 - 0.055
}

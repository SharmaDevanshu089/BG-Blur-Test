// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use window_vibrancy::apply_acrylic;
fn main() {
    bg_blur_test_lib::run()
}

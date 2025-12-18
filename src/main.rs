mod renderer;
use minifb::{Key, Window, WindowOptions};

use crate::renderer::{color::Color, framebuffer::{self, Framebuffer}, render::{self, Render}};
fn main() {
    println!("Hello, world!");
    let buffer = Framebuffer::new_as_filled(600, 800);
    let mut render: Render = Render::new(buffer);
    let mut window = Window::new(
        "Test - ESC to exit",
        render.framebuffer.width as usize,
        render.framebuffer.height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*for i in render.buffer().iter_mut() {
            *i = 0; // write something more funny here!
        }*/
        if window.is_key_down(Key::NumPad0){
            render.clear(Color::WHITE);
        }
        if window.is_key_down(Key::NumPad1){
            render.clear(Color::BLACK);
        }
        if window.is_key_down(Key::NumPad2){
            render.clear(Color::BLUE);
        }
        if window.is_key_down(Key::NumPad3){
            render.clear(Color::GREEN);
        }
        if window.is_key_down(Key::NumPad4){
            render.clear(Color::RED);
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&render.framebuffer.as_u32_buffer(), render.framebuffer.width as usize, render.framebuffer.height as usize)
            .unwrap();
    }
}

mod renderer;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::renderer::{color::{self, Color}, framebuffer::Framebuffer, render::Render};
fn main() {
    println!("Hello, world!");
    let buffer = Framebuffer::new(600, 600);
    let mut render: Render = Render::new(buffer, Color::ZERO);
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
    
    let mut count_but = 0;
    let color_array = [Color::WHITE,Color::BLACK,Color::RED,Color::GREEN,Color::BLUE];
    let mut draw_color_sel = 0;
    let mut draw_color =    color_array[draw_color_sel];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*for i in render.buffer().iter_mut() {
            *i = 0; // write something more funny here!
        }*/
        
        if window.is_key_pressed(Key::NumPad0, KeyRepeat::No){
            render.clear(Color::WHITE);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad1, KeyRepeat::No){
            render.clear(Color::BLACK);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad2, KeyRepeat::No){
            render.clear(Color::BLUE);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad3, KeyRepeat::No){
            render.clear(Color::GREEN);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad4, KeyRepeat::No){
            render.clear(Color::RED);
            count_but+=1;
        }
        if window.is_key_pressed(Key::Up, KeyRepeat::No){
            let mid_canvas= render.framebuffer.width as i32/2;    
                render.draw_line(mid_canvas, 0, mid_canvas, render.framebuffer.height as i32, 3, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::No){
            let mid_canvas = render.framebuffer.height as i32/2;
                render.draw_line(0, mid_canvas, render.framebuffer.width as i32, mid_canvas, 3, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::Left, KeyRepeat::No){
                render.draw_line(0, 0, render.framebuffer.width as i32, render.framebuffer.height as i32, 3, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::Down, KeyRepeat::No){
                render.draw_line(render.framebuffer.width as i32, 0, 0, render.framebuffer.height as i32, 3, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::RightShift, KeyRepeat::No){
                draw_color_sel += 1;
                if  draw_color_sel == 5{
                    draw_color_sel = 0;
                }
            draw_color = color_array[draw_color_sel];
            count_but+=1;
        }
        if window.is_key_pressed(Key::S, KeyRepeat::No){
            let mid_width_canvas = (render.framebuffer.width/2) as i32;
            let mid_height_canvas = (render.framebuffer.height/2) as i32;
            render.draw_rectangle(
                mid_width_canvas, 
                mid_height_canvas, 
                55, 
                55,
                false,
                3, 
                draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::C, KeyRepeat::No){
            let mid_width_canvas = (render.framebuffer.width/2) as i32;
            let mid_height_canvas = (render.framebuffer.height/2) as i32;
            render.draw_circle(mid_width_canvas, mid_height_canvas, 50, false, 3, draw_color);
            count_but += 1;
        }

        if window.is_key_pressed(Key::T, KeyRepeat::No){
            let mid_width_canvas = (render.framebuffer.width/2) as i32;
            let mid_height_canvas = (render.framebuffer.height/2) as i32;
            render.draw_triangle(
                mid_width_canvas, mid_height_canvas-50,
                mid_width_canvas-50, mid_height_canvas+50,
                mid_width_canvas+50, mid_height_canvas+50, false, 3, draw_color);
                count_but+=1;
        }
        
        if window.is_key_pressed(Key::F, KeyRepeat::No){
            let mid_width_canvas = render.framebuffer.width/2;
            let mid_height_canvas = render.framebuffer.height/2;
            render.fill(mid_width_canvas, mid_height_canvas, draw_color);
            count_but+=1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&render.framebuffer.as_u32_buffer(), render.framebuffer.width as usize, render.framebuffer.height as usize)
            .unwrap();
    }
    println!("A quantidade de vezes que os botoes foram apertados foi: {}", count_but);
}

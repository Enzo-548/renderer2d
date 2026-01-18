mod renderer;

use minifb::{HasWindowHandle, Key, KeyRepeat, Window, WindowOptions};
use image::{ImageBuffer, Rgba};
use crate::renderer::{color::{self, Color}, framebuffer::Framebuffer, render::Render, shape::Shape};

fn main() {
    println!("Hello, world!");
    let mut render: Render = Render::new(Framebuffer::new(600, 600));
    let mut window = Window::new(
        "Test - ESC to exit",
        render.layers[1].width as usize,
        render.layers[1].height as usize,
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
    let mut cur_thickness = 1;
    let mut brush_sel = 0;
    let mut out_count = 0;
    let mut display = Framebuffer::new(render.layers[1].width, render.layers[1].height);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*for i in render.buffer().iter_mut() {
            *i = 0; // write something more funny here!
        }*/
        display.update_buffer(render.layers[1].cur_buffer());
        
        if let Some(overlay) = render.overlay_mut() {
            let mut count = 0;
            for i in overlay.cur_buffer(){
                if i.a > 0 {
                    display.update_color(*i,count);
                }
                count+=1;
            } 
            overlay.clear(Color::ZERO);
        }
        
        if window.is_key_pressed(Key::NumPadEnter, KeyRepeat::Yes){
            let raw = render.layers[1].as_u8_buffer();

            let name = format!("output{}", out_count);

            let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(render.layers[1].width, render.layers[1].height, raw)
            .expect("Invalid framebuffer size");
            img.save(format!("assets/output/{name}.png")).expect("failed to save image");
            count_but += 1;
            out_count+=1;
        }
        
        if window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes) || window.is_key_pressed(Key::NumPadMinus, KeyRepeat::Yes)
        {
                if window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes){
                    cur_thickness += 1;
                }
                if window.is_key_pressed(Key::NumPadMinus, KeyRepeat::Yes){
                    cur_thickness -= 1;
                }
            
            if cur_thickness < 0  || cur_thickness > render.layers[1].width as i32{
            cur_thickness = 0;
        }
        }
        if window.is_key_pressed(Key::NumPad0, KeyRepeat::No){
            render.layers[1].clear(Color::WHITE);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad1, KeyRepeat::No){
            render.layers[1].clear(Color::BLACK);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad2, KeyRepeat::No){
            render.layers[1].clear(Color::BLUE);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad3, KeyRepeat::No){
            render.layers[1].clear(Color::GREEN);
            count_but+=1;
        }
        if window.is_key_pressed(Key::NumPad4, KeyRepeat::No){
            render.layers[1].clear(Color::RED);
            count_but+=1;
        }
        if window.is_key_pressed(Key::Up, KeyRepeat::No){
            let mid_canvas= render.layers[1].width as i32/2;    
                render.draw_line(1, mid_canvas, 0, mid_canvas, render.layers[1].height as i32, cur_thickness, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::No){
            let mid_canvas = render.layers[1].height as i32/2;
                render.draw_line(1,0, mid_canvas, render.layers[1].width as i32, mid_canvas, cur_thickness, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::Left, KeyRepeat::No){
                render.draw_line(1,0, 0, render.layers[1].width as i32, render.layers[1].height as i32, cur_thickness, draw_color);
                count_but+=1;
        }
        if window.is_key_pressed(Key::Down, KeyRepeat::No){
                render.draw_line(1, render.layers[1].width as i32, 0, 0, render.layers[1].height as i32, cur_thickness, draw_color);
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
        if window.is_key_pressed(Key::LeftShift, KeyRepeat::No){
                brush_sel += 1;
                if brush_sel > 4{
                    brush_sel = 0;
                }
            count_but+=1;
        }
        if window.is_key_pressed(Key::S, KeyRepeat::No){
            let mid_width_canvas = (render.layers[1].width/2) as i32;
            let mid_height_canvas = (render.layers[1].height/2) as i32;
            let _square = Shape::new_defined_monochrome(
                renderer::shape::ShapeKind::Rect { center: (mid_height_canvas as f32, mid_width_canvas as f32), half_w: 150.0, half_h: 150.0},
                draw_color).rasterize(&mut render, 1, cur_thickness);
        }
        if window.is_key_pressed(Key::C, KeyRepeat::No){
            let mid_width_canvas = (render.layers[1].width/2) as i32;
            let mid_height_canvas = (render.layers[1].height/2) as i32;
            let _circle = Shape::new_defined_monochrome(
                renderer::shape::ShapeKind::Circle {center: (mid_height_canvas as f32, mid_width_canvas as f32), r: 50.0 },
                draw_color,
            ).rasterize(&mut render, 1, cur_thickness);
            count_but += 1;
        }

        if window.is_key_pressed(Key::T, KeyRepeat::No){
            let mid_width_canvas = (render.layers[1].width/2) as f32;
            let mid_height_canvas = (render.layers[1].height/2) as f32;
            let _triangle = Shape::new_defined_monochrome(
                renderer::shape::ShapeKind::Polygon { vertices: vec![
                    (mid_width_canvas, mid_height_canvas-50.0),
                    (mid_width_canvas-50.0, mid_height_canvas+50.0),
                    (mid_width_canvas+50.0, mid_height_canvas+50.0)
                ]}, 
                draw_color).rasterize(&mut render, 1, cur_thickness);
            
        }
        
        if window.is_key_pressed(Key::F, KeyRepeat::No){
            let mid_width_canvas = render.layers[1].width/2;
            let mid_height_canvas = render.layers[1].height/2;
            render.bucket_fill(1, mid_width_canvas, mid_height_canvas, draw_color);
            count_but+=1;
        }
            let is_mouse_valid = window
                .get_mouse_pos(minifb::MouseMode::Pass)
                .is_some_and(|(x, y)| {
                let (w, h) = window.get_size();
                (0.0..w as f32).contains(&x) &&
                (0.0..h as f32).contains(&y)
            });

            let mut last_mouse_pos: Option<(f32, f32)> = None;
        

        pub fn draw_shape_loop( 
            shape_array: &mut Vec<Shape>,
            window: &mut Window, 
            cur_thickness: &mut i32,
            render: &mut Render,
            display: &mut Framebuffer,
            color_array: [Color; 5]
        ){
        let (mut col_sel_inline, mut col_sel_outline) = (0,0);
        let mut shape_sel = 0;
        let mut shape:&mut Shape = &mut shape_array[shape_sel];
        while !window.is_key_down(Key::Enter){
        if window.is_key_pressed(Key::LeftShift, KeyRepeat::No){
                shape_sel += 1;
                if shape_sel >= 3{
                    shape_sel = 0;
                }
        }
        shape = &mut shape_array[shape_sel];
        shape.inline_color = color_array[col_sel_inline];
        shape.outline_color = color_array[col_sel_outline];
        if window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes) || window.is_key_pressed(Key::NumPadMinus, KeyRepeat::Yes){
                if window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes){
                    *cur_thickness += 1;
                }
                if window.is_key_pressed(Key::NumPadMinus, KeyRepeat::Yes){
                    *cur_thickness -= 1;
                }
            
            if *cur_thickness < 0  || *cur_thickness > render.layers[1].width as i32{
            *cur_thickness = 0;
            }
        }
        if window.is_key_pressed(Key::Comma, KeyRepeat::No) || window.is_key_pressed(Key::Period, KeyRepeat::No){ 
            if window.is_key_pressed(Key::Comma, KeyRepeat::No){
                col_sel_inline +=1;
            }
            if window.is_key_pressed(Key::Period, KeyRepeat::No){
                col_sel_outline+=1;
                
            }
            if col_sel_inline >= color_array.len() || col_sel_outline >= color_array.len(){
                if col_sel_inline >= color_array.len(){
                    col_sel_inline = 0;
                }
                if col_sel_outline >= color_array.len(){
                    col_sel_outline = 0;
                }
            }
        }
        display.update_buffer(render.layers[1].cur_buffer());
        if let Some(overlay) = render.overlay_mut() {
            let mut count = 0;
            for i in overlay.cur_buffer(){
                if i.a > 0 {
                    display.update_color(*i,count);
                }
                count+=1;
            } 
            overlay.clear(Color::ZERO);
        }
            let arrow_pressed = {
                        window.is_key_down(Key::Up) 
                        || window.is_key_down(Key::Down)
                        || window.is_key_down(Key::Right)
                        || window.is_key_down(Key::Left)
            };
            let mut x = 0.0;
            let mut y = 0.0;
            let rot = if window.is_key_down(Key::Right) { 0.1 }
            else if window.is_key_down(Key::Left) { -0.1 }
            else { 0.0 };
                            if window.is_key_down(Key::Up){
                                y -= 1.0;
                            }
                            if window.is_key_down(Key::Down){
                                y += 1.0;
                            }
                            if window.is_key_down(Key::Right){
                                x += 1.0;
                            }
                            if window.is_key_down(Key::Left){
                                x -= 1.0;
                            }
                        if arrow_pressed && window.is_key_down(Key::M)
                        {
                            shape.translate(x, y);
                        } else if arrow_pressed && window.is_key_down(Key::S)
                        {
                            shape.scale(x*0.05,  y*0.05);
                        }else if window.is_key_pressed(Key::Right,KeyRepeat::Yes)
                            || window.is_key_pressed(Key::Left,KeyRepeat::Yes)
                        {
                            shape.rotate(rot);
                        }
                shape.rasterize(render, 0, *cur_thickness);
                window
                .update_with_buffer(&display.as_u32_buffer(), display.width as usize, display.height as usize)
                .unwrap();
            }
            (*shape).rasterize(render, 1, *cur_thickness);
        }
        if window.is_key_down(Key::X){
                                   let mid_width_canvas = (render.layers[1].width/2) as f32;
                                    let mid_height_canvas = (render.layers[1].height/2) as f32;
                                    let triangle = Shape::new_defined_polychrome(
                                        renderer::shape::ShapeKind::Polygon { vertices: vec![
                                            (mid_width_canvas, mid_height_canvas-50.0),
                                            (mid_width_canvas-50.0, mid_height_canvas+50.0),
                                            (mid_width_canvas+50.0, mid_height_canvas+50.0)
                                        ]},
                                        draw_color,
                                        Color::ZERO);
                                    let square = Shape::new_defined_polychrome(
                                    renderer::shape::ShapeKind::Rect { center: (mid_height_canvas as f32, mid_width_canvas as f32), half_w: 150.0, half_h: 150.0},
                                    draw_color,
                                    Color::ZERO);
                                    let circle = Shape::new_defined_polychrome(
                                                        renderer::shape::ShapeKind::Circle {center: (mid_height_canvas as f32, mid_width_canvas as f32), r: 50.0 },
                                                        draw_color,
                                                        Color::ZERO
                                                    );
                                    draw_shape_loop(&mut vec![triangle, square, circle],&mut window, &mut cur_thickness, &mut render, &mut display, color_array);
        }
        if is_mouse_valid || window.get_mouse_down(minifb::MouseButton::Left){
            let mut pos_vec = Vec::new();
            if window.get_mouse_down(minifb::MouseButton::Left){
                match window.get_mouse_pos(minifb::MouseMode::Discard) { 

                    Some(last_mouse_pos) => {
                        pos_vec.push(last_mouse_pos);

                        while let Some((x,y)) =  pos_vec.pop(){
                           /* if brush_sel == 4{
                                let fix_x = x;
                                let fix_y = y;
                                while window.get_mouse_down(minifb::MouseButton::Left){
                                
                                render.draw_dynam(1, brush_sel, fix_x as u32, fix_y as u32, cur_thickness, draw_color, (x as u32,y as u32));        
                                }
                            }*/
                            render.draw_dynam(1, brush_sel, x as u32, y as u32, cur_thickness, draw_color, (0,0));
                        }

                    }
                    None => {
                        println!("Coordenada inválida!");
                    }
                }
                count_but+=1;
            }
            match window.get_mouse_pos(minifb::MouseMode::Discard) { 
                
                    Some(last_mouse_pos) => {
                        pos_vec.push(last_mouse_pos);
                        while let Some((x,y)) =  pos_vec.pop(){
                            if brush_sel == 3 {

                            let base_buffer = {
                                let buf = render.layers[1].cur_buffer();
                                buf.to_vec()
                            };

                            render.layers[0].update_buffer(&base_buffer);

                            render.draw_dynam(0, brush_sel, x as u32, y as u32, cur_thickness, draw_color,(0,0));
                            }else if brush_sel == 4{
                            brush_sel = 0;
                            }else{
                                    render.draw_dynam(0, brush_sel, x as u32, y as u32, cur_thickness, draw_color,(0,0));
                                    }
                    }
                    } None => {
                        println!("Coordenada inválida!");
                    }
                }            
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&display.as_u32_buffer(), display.width as usize, display.height as usize)
            .unwrap();
    }
    println!("A quantidade de vezes que os botoes foram apertados foi: {}", count_but);
}

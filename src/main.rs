extern crate sdl2;

use sdl2::pixels::Color;

mod gametime;
mod video;

fn main() {
    let mut sdl_ctx = sdl2::init().video().unwrap();
    
    let window = sdl_ctx.window("Rust Doom", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        
    let mut renderer = window.renderer().present_vsync().build().unwrap();

    let mut running = true;
    let mut gametime = gametime::GameTime::new();
    
    let mut video_ctx = video::Video::new(640, 480);
    
    let vx1 = 70;
    let vy1 = 20;
    let vx2 = 70;
    let vy2 = 70;
    
    let mut px = 50;
    let mut py = 50;
    let mut angle = 0_f32;
    
    while running {
        for event in sdl_ctx.event_pump().poll_iter() {
            use sdl2::event::Event;
            use sdl2::keycode::KeyCode;
            
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: KeyCode::Escape, ..} => {
                    running = false
                },
                _ => {}
            }
        }
               
        gametime.update();
               
        // Render
        renderer.drawer().clear();

        video_ctx.clear();

        video_ctx.set_draw_color(Color::RGB(255, 0, 255));
        video_ctx.line(200, 50, 201, 100);
        video_ctx.line(210, 50, 210, 100);
        video_ctx.line(250, 50, 300, 51);
        video_ctx.line(250, 60, 300, 60);

        video_ctx.set_draw_color(Color::RGB(255, 255, 0));
        video_ctx.line(vx1, vy1, vx2, vy2);
        video_ctx.line(px, py, ((angle.cos()*20_f32) as i32) + px, ((angle.sin()*20_f32) as i32) + py);
        
        angle += 1_f32 * gametime.dt();
        
        video_ctx.render(&mut renderer);
        
        renderer.drawer().present();
    }
}
extern crate sdl2;
extern crate gl;

use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::time::Duration;
use std::io::{self, Write};
use std::{thread, time::*};
use std::cmp;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn settcolor(r: u8, g: u8, b:u8, tex:&mut sdl2::render::Texture<'_>){
    tex.set_color_mod(r, g, b);
}

fn main() {

    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ASCII-Game", screen_width, screen_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    

    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .target_texture()
        .accelerated()
        .build()
        .unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();


    let texture_creator = canvas.texture_creator();
    
    let surface = Surface::load_bmp("cga8.bmp").unwrap();
    let mut font_texture = surface.as_texture(&texture_creator).unwrap();
    let mut texture = texture_creator
    .create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 256)
    .map_err(|e| e.to_string()).unwrap();
    //unsafe{
        //texture.gl_bind_texture();
    //}

    
    canvas.present();
    /* For using openGL directly

    // initialization
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // sdl::render creates a context for you, if you use a Canvas you need to use it.
    canvas.window().gl_set_context_to_current();

    // ... in the main loop ...
    unsafe {
        gl::ClearColor(0.6, 0.0, 0.8, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    canvas.present();
     */

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
   'running: loop {
        let now = Instant::now();
        i = (i % 255)+1;
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.copy(&texture, None, None).unwrap();

        for x in 0..100{
            for y in 0..75{
                settcolor(x as u8, y as u8, ((x*y)/i) as u8, &mut font_texture);
                canvas.copy(&font_texture, Rect::new(16+8,0,8,8), Rect::new(8*x,8*y,8,8)).unwrap();
            }
        }
        
        canvas.present();
        // Cap to 60 fps
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        let fps = 1000/cmp::max(1,(now.elapsed().as_micros()/1000));
        println!("FPS is: {}", fps);
    }

    /*let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    let alpha: [char; 5] = ['.', '-', '=', '#', '@'];
    let mut counter = 0;
    let ten_millis = time::Duration::from_millis(100);
    let mut framerate = 0;
    while(true){
        let buf: [u8; 3600] = [alpha[counter] as u8; 3600];
        let ar:String = String::from_utf8_lossy(&buf).into_owned();
        write!(handle, "{:}", ar); // add `?` if you care about errors here
        //println!("{:}", ar);
        counter = (counter + 1) % 5;
        thread::sleep(ten_millis);
    }*/
}

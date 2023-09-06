extern crate sdl2;

use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use std::time::Duration;
use std::io::{self, Write};
use std::{thread, time};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
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
        .build()
        .unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();


    let texture_creator = canvas.texture_creator();
    
    let surface = Surface::load_bmp("cga8.bmp").unwrap();
    let font_texture = surface.as_texture(&texture_creator).unwrap();
    let mut texture = texture_creator
    .create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 256)
    .map_err(|e| e.to_string()).unwrap();
    unsafe{
        texture.gl_bind_texture();
    }
    
    
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
        i = (i + 1) % 255;
        canvas.clear();
        println!("{:#?}",texture.query());
        texture.with_lock(None, |buffer:&mut [u8], pitch: usize| {
            for y in 0..256{
                for x in 0..256{
                    let offset = y*pitch + x * 4;
                    buffer[offset] = 100 as u8;
                    buffer[offset + 1] = 0 as u8;
                    buffer[offset + 2] = x as u8;
                    buffer[offset + 3] = i as u8;
                }
            }
        }).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.copy(&font_texture, None, None).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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

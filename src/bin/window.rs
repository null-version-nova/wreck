use std::error::Error;
use sdl3::{self, event::Event};

fn main() -> Result<(),Box<dyn Error>> {
    let process = sdl3::init()?;
    let video = process.video()?;
    let mut events = process.event_pump()?;
    let mut canvas = video.window_and_renderer("meow", 1280, 720)?;
    'running: loop {
        canvas.clear();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.present();
    }
    Ok(())
}
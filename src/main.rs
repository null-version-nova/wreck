use std::error::Error;

use wreck::display::process::{Process, Processing};

fn main() -> Result<(),Box<dyn Error>> {
    {
        let mut render_manager = wreck::display::rendermanager::INSTANCE.lock()?;
        render_manager.get_new_window("meow", 1280, 720)?;
    }
    let mut process = wreck::display::process::Game::new(0.016,Box::new(|delta|{
        let _ = wreck::display::rendermanager::INSTANCE.lock().unwrap().process(delta);
    }));
    process.run();
    Ok(())
}
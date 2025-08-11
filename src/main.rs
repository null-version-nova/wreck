use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let mut render_manager = wreck::display::rendermanager::INSTANCE.lock()?;
    let mut window = render_manager.get_new_window("meow", 1280, 720)?;
    Ok(())
}
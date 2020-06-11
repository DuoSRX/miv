use std::env;
use miv::state::State;
use miv::view::View;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

fn event_loop(state: &mut State, view: &mut View) -> Result<()> {
    loop {
        let event = read()?;

        match event {
            Event::Key(ek) => {
                match ek.code {
                    KeyCode::Char('q') => break,
                    _ => {
                        let exit = state.handle_key(ek);
                        if exit { break }
                    }
                }
            }
            _ => {}
        }

        view.render(state);
    }

    Ok(())
}


fn main() -> Result<()> {
    enable_raw_mode()?;

    let (width, height) = crossterm::terminal::size().unwrap();
    let mut state = State::new(width as usize, height as usize);
    let mut view = View::new();

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        state.buffer.borrow_mut().load_file(path.clone());
        state.buffer.borrow_mut().filepath = Some(path.clone());
    }

    view.render(&state);

    if let Err(e) = event_loop(&mut state, &mut view) {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}

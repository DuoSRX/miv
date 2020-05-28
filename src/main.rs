use std::env;
use miv::state::State;
use miv::view::View;
use std::io::{stdout, Write};
use crossterm::{
    cursor::position,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
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

// fn main() {
//     let mut options = rustbox::InitOptions::default();
//     options.output_mode = rustbox::OutputMode::EightBit;
//     options.buffer_stderr = true;
//     let rustbox = RustBox::init(options).unwrap();

//     let mut view = View::new(&rustbox);
//     let mut state = State::new(rustbox.width(), rustbox.height());

//     let args: Vec<String> = env::args().collect();
//     if let Some(path) = args.get(1) {
//         state.buffer.borrow_mut().load_file(path.clone());
//         state.buffer.borrow_mut().filepath = Some(path.clone());
//     }

//     rustbox.clear();
//     rustbox.set_cursor(0, 0);
//     view.render(&state);

//     'running: loop {
//         match rustbox.poll_event(false) {
//             Ok(rustbox::Event::KeyEvent(Key::Ctrl('c'))) => {
//                 break 'running;
//             }
//             Ok(rustbox::Event::KeyEvent(key)) => {
//                 let exit = state.handle_key(key);
//                 if exit { break 'running }
//             }
//             Ok(rustbox::Event::ResizeEvent(w, h)) => {
//                 state.width = w as usize;
//                 state.height = h as usize;
//             }
//             Err(e) => panic!("{}", e),
//             _ => {}
//         }

//         view.render(&state);
//     }
// }

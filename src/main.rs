//use enigo::*;
use rdev::{simulate, EventType, Key, SimulateError};
use sdl2::event::Event;
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
enum Stroke {
    Handstroke,
    Backstroke,
}

fn send_key(key: Key) {
    send(&EventType::KeyPress(key));
    send(&EventType::KeyRelease(key));
}

fn send_shift_key(key: Key) {
    send(&EventType::KeyPress(Key::ShiftLeft));
    send_key(key);
    send(&EventType::KeyRelease(Key::ShiftLeft));
}

fn send_function_key(key: Key) {
    send(&EventType::KeyPress(Key::Function));
    send_key(key);
    send(&EventType::KeyRelease(Key::Function));
}

fn key_click(k: char) {
    match k {
        '1' => send_function_key(Key::F1),
        '2' => send_function_key(Key::F2),
        '5' => send_function_key(Key::F5),
        '6' => send_function_key(Key::F6),
        'j' => send_key(Key::KeyJ),
        'f' => send_key(Key::KeyF),
        'b' => send_key(Key::KeyB),
        'n' => send_key(Key::KeyN),
        'g' => send_key(Key::KeyG),
        'S' => send_shift_key(Key::KeyS),
        _ => panic!("Unrecognised key {}", k),
    }
}

fn send(event_type: &EventType) {
    let delay = Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn joystick() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let joystick_subsystem = sdl_context.joystick()?;

    let available = joystick_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);
    let mut joysticks = Vec::new();
    for id in 0..available {
        joysticks.push(joystick_subsystem.open(id).unwrap());
    }

    let key = ['j', 'f'];
    // let button = [['b', 'n'], ['g', 'S']]; // Ringing Room
    let button = [['5', '6'], ['1', '2']]; // Mabel
    let debounce = 400;
    let mut last_val = [0.0, 0.0];
    let mut last_time = [0, 0];
    let mut last_stroke = [Stroke::Backstroke, Stroke::Backstroke];
    //   let mut enigo = Enigo::new();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::JoyAxisMotion {
                    timestamp,
                    which,
                    axis_idx,
                    value,
                } => {
                    if axis_idx == 2 {
                        // Z axis
                        let bell = which as usize;
                        let val = value as f32 / 16960.0;
                        let lower_pos = -0.8;
                        let upper_pos = 0.0;
                        if val < lower_pos
                            && last_val[bell] >= lower_pos
                            && last_stroke[bell] == Stroke::Handstroke
                            && timestamp - last_time[bell] > debounce
                        {
                            key_click(key[bell]);
                            last_stroke[bell] = Stroke::Backstroke;
                            last_time[bell] = timestamp;
                        } else if val > upper_pos
                            && last_val[bell] <= upper_pos
                            && last_stroke[bell] == Stroke::Backstroke
                            && timestamp - last_time[bell] > debounce
                        {
                            key_click(key[bell]);
                            last_stroke[bell] = Stroke::Handstroke;
                            last_time[bell] = timestamp;
                        }
                        last_val[bell] = val;
                    }
                }
                Event::JoyButtonDown {
                    which, button_idx, ..
                } => {
                    let bell = which as usize;
                    let side = button_idx as usize;
                    let c = button[bell][side];
                    key_click(c);
                }
                Event::Quit { .. } => break 'running,
                _ => (),
            }
        }
    }

    Ok(())
}

pub fn main() {
    joystick().unwrap();
}

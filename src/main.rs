use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use sdl2::event::Event;

#[derive(PartialEq)]
enum Stroke {
    Handstroke,
    Backstroke,
}

struct Buttons {
    left: char,
    right: char,
}

struct Bell {
    key: char,
    buttons: Buttons,
}

struct Strike {
    stroke: Stroke,
    time: u32,
}

fn key_click(enigo: &mut Enigo, c: char) {
    enigo.key(Key::Unicode(c), Click).unwrap();
}

fn run(ebells: &[Bell]) -> Result<(), String> {
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
    let debounce = 450;
    let left_bell_strike = Strike {
        stroke: Stroke::Backstroke,
        time: 0,
    };
    let right_bell_strike = Strike {
        stroke: Stroke::Backstroke,
        time: 0,
    };
    let mut last_strike = [right_bell_strike, left_bell_strike];
    let mut last_position = [0.0, 0.0];
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
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
                        let position = value as f32 / 16960.0;
                        let lower_limit = -0.8;
                        let upper_limit = 0.0;
                        let lower_limit_reached =
                            position < lower_limit && last_position[bell] >= lower_limit;
                        let upper_limit_reached =
                            position > upper_limit && last_position[bell] <= upper_limit;
                        let debounce_ok = timestamp - last_strike[bell].time > debounce;
                        if debounce_ok {
                            if lower_limit_reached && last_strike[bell].stroke == Stroke::Handstroke
                            {
                                key_click(&mut enigo, ebells[bell].key);
                                last_strike[bell] = Strike {
                                    stroke: Stroke::Backstroke,
                                    time: timestamp,
                                };
                            } else if upper_limit_reached
                                && last_strike[bell].stroke == Stroke::Backstroke
                            {
                                key_click(&mut enigo, ebells[bell].key);
                                last_strike[bell] = Strike {
                                    stroke: Stroke::Handstroke,
                                    time: timestamp,
                                };
                            }
                        }
                        last_position[bell] = position;
                    }
                }
                Event::JoyButtonDown {
                    which, button_idx, ..
                } => {
                    let bell = which as usize;
                    let c = match button_idx {
                        0 => ebells[bell].buttons.left,
                        1 => ebells[bell].buttons.right,
                        _ => panic!("button unknown"),
                    };
                    key_click(&mut enigo, c);
                }
                Event::Quit { .. } => break 'running,
                _ => (),
            }
        }
    }

    Ok(())
}

pub fn main() {
    let left_bell = Bell {
        key: 'f',
        buttons: Buttons {
            left: ';',  // Single
            right: 'a', // Bob
        },
    };

    let right_bell = Bell {
        key: 'j',
        buttons: Buttons {
            left: 'g',  // Go
            right: '.', // Stop
        },
    };
    let ebells = [left_bell, right_bell];
    run(&ebells).unwrap();
}

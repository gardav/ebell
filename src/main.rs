use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use sdl2::event::Event;

#[derive(PartialEq)]
enum Stroke {
    Handstroke,
    Backstroke,
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
    let debounce = 450;
    let mut last_val = [0.0, 0.0];
    let mut last_time = [0, 0];
    let mut last_stroke = [Stroke::Backstroke, Stroke::Backstroke];
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
                        let val = value as f32 / 16960.0;
                        let lower_pos = -0.8;
                        let upper_pos = 0.0;
                        if val < lower_pos
                            && last_val[bell] >= lower_pos
                            && last_stroke[bell] == Stroke::Handstroke
                            && timestamp - last_time[bell] > debounce
                        {
                            enigo.key(Key::Unicode(key[bell]), Click).unwrap();
                            last_stroke[bell] = Stroke::Backstroke;
                            last_time[bell] = timestamp;
                        } else if val > upper_pos
                            && last_val[bell] <= upper_pos
                            && last_stroke[bell] == Stroke::Backstroke
                            && timestamp - last_time[bell] > debounce
                        {
                            enigo.key(Key::Unicode(key[bell]), Click).unwrap();
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
                    enigo.key(Key::Unicode(c), Click).unwrap();
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

use enigo::*;
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
    let button = [['b', 'n'], ['g', 'S']];
    let debounce = 450;
    let mut last_val = [0.0, 0.0];
    let mut last_time = [0, 0];
    let mut last_stroke = [Stroke::Backstroke, Stroke::Backstroke];
    let mut enigo = Enigo::new();
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
                        if val < -0.9
                            && last_val[bell] >= -0.9
                            && last_stroke[bell] == Stroke::Handstroke
                            && timestamp - last_time[bell] > debounce
                        {
                            enigo.key_click(Key::Layout(key[bell]));
                            last_stroke[bell] = Stroke::Backstroke;
                            last_time[bell] = timestamp;
                        } else if val > 0.0
                            && last_val[bell] <= 0.0
                            && last_stroke[bell] == Stroke::Backstroke
                        {
                            enigo.key_click(Key::Layout(key[bell]));
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
                    if c == '+' {
                        enigo.key_sequence_parse("{+SHIFT}={-SHIFT}")
                    } else if c.is_uppercase() {
                        enigo.key_sequence_parse(
                            &["{+SHIFT}", "{-SHIFT}"].join(&c.to_lowercase().to_string()),
                        )
                    } else if c == '\u{f704}' {
                        enigo.key_click(Key::F4)
                    } else {
                        enigo.key_click(Key::Layout(c))
                    }
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

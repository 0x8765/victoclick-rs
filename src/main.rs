use device_query::{DeviceQuery, DeviceState, Keycode};
use rdev::{simulate, Button, EventType, SimulateError};
use std::{thread, time};


fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }

    if std::env::consts::OS == "macos" {
        thread::sleep(time::Duration::from_micros(300)); 
    }
}

fn click() {
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}

fn main() {
    println!("-----------");
    println!("VICTOCLICK - autoclicker");
    println!("by 0x876");
    println!("-----------");
    println!("very fast auto clicker");
    println!("(1) - Toggle mod, on/off");
    println!("(2) - Hold mod, press for click (untested on Windows)");

    let clickmod;
    loop {
        println!("what do you want (1 or 2): ");
        let input = read_string();
        match input.trim().parse::<u32>() {
            Ok(number) if number == 1 || number == 2 => {
                clickmod = number;
                println!("Mod {number} loaded, Ctrl + C to stop.");
                break;
            }
            _ => {
                println!("It not a valid choice.");
            }
        }

    }
    
    let mut onoff = false;

    let device_state = DeviceState::new();
    
    loop {
        if clickmod == 1 && onoff {
            click();
        }
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::J) {
            if clickmod == 1 {
                onoff = !onoff;
                thread::sleep(time::Duration::from_millis(300));
            } else if clickmod == 2 {
                click();
            }
        } else if keys.contains(&Keycode::J){
            onoff = false;
            thread::sleep(time::Duration::from_millis(300));
        }
    }
 
}

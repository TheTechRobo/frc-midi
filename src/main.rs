use midi_control::MidiMessage;

use std::sync::mpsc::{channel, Sender};
use std::net::TcpStream;
use std::io::prelude::*;

use midi_control::MidiMessage::*;

use serde::Serialize;

mod types;
use types::*;

const WORLDE_EASY_KEY: &str = "WORLDE easy key";

fn find_port<T>(midi_io: &T) -> Option<T::Port>
where T: midir::MidiIO, {
    let mut device_port: Option<T::Port> = None;
    for port in midi_io.ports() {
        if let Ok(port_name) = midi_io.port_name(&port) {
            if port_name.contains(WORLDE_EASY_KEY) {
                device_port = Some(port);
                break;
            }
        }
    }
    device_port
}

#[derive(Debug, Serialize)]
struct Retval {
    d: ControllerEvent,
    timestamp_us: u64
}

struct Stuff {
    sender: Sender<Retval>,
    last_program: u8,
    dial_calibrated: bool,
}

fn midi_message_callback(timestamp: u64, data: &[u8], df: &mut Stuff) {
    let sender = &mut df.sender;
    let msg: MidiMessage = MidiMessage::from(data);
    //println!("thread callback: {}: received {:?} => {:?}", timestamp, data, msg);
    let event: Option<ControllerEvent> = match msg {
        NoteOn(_, key) => Some(ControllerEvent::ButtonPress(Button::from_int(key.key))),
        NoteOff(_, key) => Some(ControllerEvent::ButtonRelease(Button::from_int(key.key))),
        ControlChange(_, event) => {
            let rv;
            if event.control == 1 {
                rv = match event.value {
                    0 => Some(ControllerEvent::ButtonRelease(Button::ButtonMod)),
                    127 => Some(ControllerEvent::ButtonPress(Button::ButtonMod)),
                    _ => None
                }
            } else {
                eprintln!("Received unknown contorl code {event:?}");
                rv = None;
            }
            rv
        },
        Invalid => {
            let rv: Option<ControllerEvent>;
            if data[0] == 192 {
                // Program change (dial)
                let direction: DialMovement;
                if data[1] == 0 || data[1] < df.last_program {
                    direction = DialMovement::Left;
                } else if data[1] == 127 || data[1] > df.last_program {
                    direction = DialMovement::Right;
                } else {
                    return;
                }
                df.last_program = data[1];
                if df.dial_calibrated {
                    rv = Some(ControllerEvent::DialTurn(direction));
                } else {
                    df.dial_calibrated = true;
                    rv = None;
                }
            } else {
                rv = None;
            }
            rv
        },
        PitchBend(..) => None,
        _ => {
            eprintln!("received unknown input {msg:?}");
            None
        }
    };
    if let Some(ev) = event {
        let rv = Retval {
            d: ev,
            timestamp_us: timestamp
        };
        sender.send(rv).expect("failed to send sending stuff sender");
    }
}

fn main() {
    let midi_input = midir::MidiInput::new("ForTheMemes").unwrap();
    let device_port = find_port(&midi_input);
    if device_port.is_none() {
        eprintln!("Input device not found!");
        return;
    }
    let (sender, receiver) = channel::<Retval>();

    let device_port = device_port.unwrap();
    let thing: Stuff = Stuff {
        sender,
        last_program: 0,
        dial_calibrated: false,
    };
    let _connect_in = midi_input.connect(
        &device_port,
        WORLDE_EASY_KEY,
        midi_message_callback,
        thing
    );
    let mut can_go = false;

    eprintln!("Waiting for server welcome...");
    let mut stream;
    loop {
        let thing = TcpStream::connect("127.0.0.1:7757");
        match thing {
            Ok(r) => {
                stream = r;
            },
            Err(e) => {
                eprintln!("failed to connect to relay: {e}");
                continue;
            }
        };
        match stream.read(&mut [0;1]) {
            std::io::Result::Ok(_) => break,
            std::io::Result::Err(e) => {
                eprintln!("didnt receive welcome message ({e}), reconnecting");
                let _ = stream.shutdown(std::net::Shutdown::Both);
                continue;
            }
        }
    }

    eprintln!("Press the MOD button on the keyboard to activate the controller.");
    eprintln!("Press CTRL-C on the computer to end.");

    loop {
        let msg: Retval = receiver.recv().unwrap();
        if !can_go {
            if let ControllerEvent::ButtonRelease(btn) = msg.d {
                if btn == Button::ButtonMod {
                    can_go = true;
                    eprintln!("Controller activated!");
                }
            }
            continue;
        }
        match stream.write_all(&[msg.d.to_bytes()]) {
            Ok(_) => (),
            Err(e) => eprintln!("warning: cannot send to stream: {e}; try restarting the controller client"),
        }
    }
}

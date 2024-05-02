use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Button {
    ButtonC,
    ButtonCS,
    ButtonD,
    ButtonDS,
    ButtonE,
    ButtonF,
    ButtonFS,
    ButtonG,
    ButtonGS,
    ButtonA,
    ButtonAS,
    ButtonB,
    ButtonMod
}
pub use Button::*;

impl Button {
    pub fn to_key(&self) -> u8 {
        match self {
            ButtonC => 0,
            ButtonCS => 1,
            ButtonD => 2,
            ButtonDS => 3,
            ButtonE => 4,
            ButtonF => 5,
            ButtonFS => 6,
            ButtonG => 7,
            ButtonGS => 8,
            ButtonA => 9,
            ButtonAS => 10,
            ButtonB => 11,
            ButtonMod => 12,
        }
    }
    pub fn from_int(key: u8) -> Button {
        let nk: u8 = key % 12;
        // Octave is not currently necessary
        // let octave = key / 12;
        assert!(nk < 12);
        let b: Button = match nk {
            0 => Button::ButtonC,
            1 => Button::ButtonCS,
            2 => Button::ButtonD,
            3 => Button::ButtonDS,
            4 => Button::ButtonE,
            5 => Button::ButtonF,
            6 => Button::ButtonFS,
            7 => Button::ButtonG,
            8 => Button::ButtonGS,
            9 => Button::ButtonA,
            10 => Button::ButtonAS,
            11 => Button::ButtonB,
            _ => unreachable!() // I stand by the fact that this should not be necessary
        };
        b
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note2name = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "MOD"];
        let tk: usize = self.to_key().into();
        write!(f, "{}", note2name[tk])
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DialMovement {
    Left,
    Right,
}

impl std::fmt::Display for DialMovement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            DialMovement::Left => "Left",
            DialMovement::Right => "Right",
        };
        write!(f, "{a}")
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ControllerEvent {
    ButtonPress(Button),
    ButtonRelease(Button),
    DialTurn(DialMovement)
}

impl std::fmt::Display for ControllerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControllerEvent::ButtonPress(e) => write!(f, "ButtonPress({e})"),
            ControllerEvent::ButtonRelease(e) => write!(f, "ButtonRelease({e})"),
            ControllerEvent::DialTurn(e) => write!(f, "DialTurn({e})")
        }
    }
}

const BUTTON_PRESS:   u8 = 0b01000000;
const BUTTON_RELEASE: u8 = 0b11000000;
const DIAL_MOVEMENT:  u8 = 0b10000000;

impl ControllerEvent {
    pub fn to_bytes(&self) -> u8 {
        let mut res;
        match self {
            ControllerEvent::ButtonPress(button) => {
                res = BUTTON_PRESS;
                res |= button.to_key();
            },
            ControllerEvent::ButtonRelease(button) => {
                res = BUTTON_RELEASE;
                res |= button.to_key();
            },
            ControllerEvent::DialTurn(event) => {
                res = DIAL_MOVEMENT;
                if *event == DialMovement::Left {
                    res |= 1;
                }
            }
        };
        res
    }
}

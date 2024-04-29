#[derive(Debug, Eq, PartialEq)]
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
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note2name = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "MOD"];
        let tk: usize = self.to_key().into();
        write!(f, "{}", note2name[tk])
    }
}

impl Button {
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

#[derive(Debug)]
pub enum DialMovement {
    Left,
    Right,
    NoChange
}

impl std::fmt::Display for DialMovement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            DialMovement::Left => "Left",
            DialMovement::Right => "Right",
            DialMovement::NoChange => "No Change"
        };
        write!(f, "{a}")
    }
}

#[derive(Debug)]
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

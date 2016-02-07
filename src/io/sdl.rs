use sdl2::EventPump;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

use std::rc::Rc;
use std::cell::RefCell;

use io::IO;
use memory::MemSegment;
use util::ShiftRegister8;

///Some bits of the controller reads return open bus garbage. Since the last byte on the bus is
///almost always 0x40, we can just use that as a constant for now.
const OPEN_BUS: u8 = 0x40;

const A:      u8 = 1 << 0;
const B:      u8 = 1 << 1;
const SELECT: u8 = 1 << 2;
const START:  u8 = 1 << 3;
const UP:     u8 = 1 << 4;
const DOWN:   u8 = 1 << 5;
const LEFT:   u8 = 1 << 6;
const RIGHT:  u8 = 1 << 7;

pub struct SdlIO {
    event_pump: Rc<RefCell<EventPump>>,
    strobe: bool,
    controller1: ShiftRegister8,
    controller2: ShiftRegister8,
}

impl SdlIO {
    pub fn new(pump: Rc<RefCell<EventPump>> ) -> SdlIO {
        SdlIO {
            event_pump: pump,
            strobe: false,
            controller1: ShiftRegister8::new(0),
            controller2: ShiftRegister8::new(0),
        }
    }
}

impl MemSegment for SdlIO {
    fn read(&mut self, idx: u16) -> u8 {
        match idx {
            0x4016 => OPEN_BUS | self.controller1.shift(),
            0x4017 => OPEN_BUS | self.controller2.shift(),
            x => invalid_address!(x),
        }
    }

    fn write(&mut self, idx: u16, val: u8) {
        match idx {
            0x4016 => self.strobe = val & 0x01 != 0,
            0x4017 => (),
            x => invalid_address!(x),
        }
    }
}

fn read_key(state: &KeyboardState, key: Scancode, val: u8) -> u8 {
    if state.is_scancode_pressed(key) {
        val
    }
    else {
        0
    }
}

impl IO for SdlIO {
    fn poll(&mut self) {
        if !self.strobe {
            return;
        }
        
        let pump_ref = self.event_pump.borrow();
        let state = KeyboardState::new(&*pump_ref);
        
        let c1 = 0 | 
            read_key(&state, Scancode::Z, A) |
            read_key(&state, Scancode::X, B) |
            read_key(&state, Scancode::Return, START) |
            read_key(&state, Scancode::Backspace, SELECT) |
            read_key(&state, Scancode::Up, UP) |
            read_key(&state, Scancode::Down, DOWN) |
            read_key(&state, Scancode::Right, RIGHT) |
            read_key(&state, Scancode::Left, LEFT);
        self.controller1.load(c1);
    }
}
use crate::system::win32::*;

pub struct Input {
    pub controllers: [Controller; XUSER_MAX_COUNT as usize],
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl Input {
    pub fn new() -> Self {
        Input {
            controllers: [
                Controller::new(),
                Controller::new(),
                Controller::new(),
                Controller::new(),
            ],
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn poll(&mut self, hwnd: HWND) {
        self.poll_controllers();
        self.poll_mouse_position(hwnd)
    }

    pub fn advance(&mut self) {
        self.set_controllers();
        self.mouse.advance_input();
        self.keyboard.advance_input();
    }

    pub fn process_messages(&mut self, message: u32, wparam: WPARAM) -> LPARAM {
        match message {
            WM_LBUTTONDOWN => {
                self.set_mouse_state(MOUSECODE::L as u8);
                0
            }
            WM_LBUTTONUP => {
                self.release_mouse_state(MOUSECODE::L as u8);
                0
            }
            WM_RBUTTONDOWN => {
                self.set_mouse_state(MOUSECODE::R as u8);
                0
            }
            WM_RBUTTONUP => {
                self.release_mouse_state(MOUSECODE::R as u8);
                0
            }
            WM_MBUTTONDOWN => {
                self.set_mouse_state(MOUSECODE::M as u8);
                0
            }
            WM_MBUTTONUP => {
                self.release_mouse_state(MOUSECODE::M as u8);
                0
            }
            WM_XBUTTONDOWN => {
                if wparam & (0x1 << 16) != 0 {
                    self.set_mouse_state(MOUSECODE::X as u8);
                }
                if wparam & (0x2 << 16) != 0 {
                    self.set_mouse_state(MOUSECODE::Y as u8);
                }
                0
            }
            WM_XBUTTONUP => {
                if wparam & (1 << 16) != 0 {
                    self.release_mouse_state(MOUSECODE::X as u8);
                }
                if wparam & (2 << 16) != 0 {
                    self.release_mouse_state(MOUSECODE::Y as u8);
                }
                0
            }
            WM_MOUSEWHEEL => {
                self.set_wheel_delta((wparam >> 16) as i16);
                0
            }
            WM_MOUSEHWHEEL => {
                self.set_wheel_h_delta((wparam >> 16) as i16);
                0
            }
            WM_SYSKEYDOWN => {
                let ch: VKCODE = wparam as u32;
                self.set_key_state(ch as usize);
                0
            }
            WM_SYSKEYUP => {
                let ch: VKCODE = wparam as u32;
                self.release_key_state(ch as usize);
                0
            }
            WM_KEYDOWN => {
                let ch: VKCODE = wparam as u32;
                self.set_key_state(ch as usize);
                0
            }
            WM_KEYUP => {
                let ch: VKCODE = wparam as u32;
                self.release_key_state(ch as usize);
                0
            }
            _ => 0,
        }
    }

    pub fn poll_controllers(&mut self) {
        for controller_index in 0..1 {
            let controller_state = XINPUT_STATE::default();

            unsafe {
                if XInputGetState(controller_index, &controller_state) == ERROR_SUCCESS {
                    self.controllers[controller_index as usize].process_input(&controller_state);
                    //println!("controller state: {:?}", &controller_state);
                } else {
                }
            }
        }
    }

    pub fn poll_mouse_position(&mut self, hwnd: HWND) {
        unsafe {
            GetCursorPos(&self.mouse.position);
            ScreenToClient(hwnd, &self.mouse.position);
        }
    }

    pub fn set_mouse_state(&mut self, button: u8) {
        self.mouse.current_frame |= button;
    }

    pub fn release_mouse_state(&mut self, button: u8) {
        self.mouse.current_frame ^= button;
    }

    pub fn set_wheel_delta(&mut self, wheel_delta: i16) {
        self.mouse.wheel_delta = wheel_delta;
    }

    pub fn set_wheel_h_delta(&mut self, wheel_h_delta: i16) {
        self.mouse.wheel_h_delta = wheel_h_delta;
    }

    pub fn set_key_state(&mut self, index: usize) {
        self.keyboard.current_frame[index] = true;
    }

    pub fn release_key_state(&mut self, index: usize) {
        self.keyboard.current_frame[index] = false;
    }

    pub fn set_controllers(&self) {
        for controller_index in 0..XUSER_MAX_COUNT {
            unsafe {
                XInputSetState(
                    controller_index,
                    &self.controllers[controller_index as usize].vibration,
                );
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum KEYCODE {
    BACK = 8,
    TAB = 9,

    ENTER = 13,

    SHIFT = 16,
    CTRL = 17,
    ALT = 18, //SYSKEY
    PAUSE = 19,
    CAPS = 20,

    ESC = 27,

    SPACE = 32,
    PAGE_UP = 33,
    PAGE_DOWN = 34,
    END = 35,
    HOME = 36,
    LEFT = 37,
    UP = 38,
    RIGHT = 39,
    DOWN = 40,

    INS = 45,
    DEL = 46,

    NUM0 = 48,
    NUM1 = 49,
    NUM2 = 50,
    NUM3 = 51,
    NUM4 = 52,
    NUM5 = 53,
    NUM6 = 54,
    NUM7 = 55,
    NUM8 = 56,
    NUM9 = 57,

    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,

    PAD0 = 96,
    PAD1 = 97,
    PAD2 = 98,
    PAD3 = 99,
    PAD4 = 100,
    PAD5 = 101,
    PAD6 = 102,
    PAD7 = 103,
    PAD8 = 104,
    PAD9 = 105,
    PAD_MULT = 106,
    PAD_ADD = 107,

    PAD_SUB = 109,
    PAD_DEC = 110,
    PAD_DIV = 111,

    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121, //SYSKEY
    F11 = 122,
    F12 = 123,

    SEMI = 186,
    PLUS = 187,
    COMMA = 188,
    MINUS = 189,
    PERIOD = 190,
    FORWARD_SLASH = 191,
    TILDE = 192,

    OPEN_BRACKET = 219,
    BACK_SLASH = 220,
    CLOSE_BRACKET = 221,
    QUOTE = 222,
}

pub struct Keyboard {
    current_frame: [bool; 256],
    previous_frame: [bool; 256],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            current_frame: [false; 256],
            previous_frame: [false; 256],
        }
    }

    pub fn advance_input(&mut self) {
        self.previous_frame = self.current_frame;
    }

    pub fn pressed(&self, key_code: KEYCODE) -> bool {
        self.current_frame[key_code as usize] && !self.previous_frame[key_code as usize]
    }

    pub fn held(&self, key_code: KEYCODE) -> bool {
        self.current_frame[key_code as usize]
    }

    pub fn released(&self, key_code: KEYCODE) -> bool {
        !self.current_frame[key_code as usize] && self.previous_frame[key_code as usize]
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MOUSECODE {
    L = 1,
    R = 2,
    M = 4,
    X = 8,
    Y = 16,
}

pub struct Mouse {
    position: POINT,
    current_frame: u8,
    previous_frame: u8,
    wheel_delta: i16,
    wheel_h_delta: i16,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            position: POINT::default(),
            current_frame: 0,
            previous_frame: 0,
            wheel_delta: 0,
            wheel_h_delta: 0,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.position.x
    }

    pub fn get_y(&self) -> i32 {
        self.position.y
    }

    pub fn get_wheel_delta(&self) -> i16 {
        self.wheel_delta
    }

    pub fn get_wheel_h_delta(&self) -> i16 {
        self.wheel_h_delta
    }

    pub fn pressed(&self, mouse_code: MOUSECODE) -> bool {
        self.current_frame & mouse_code as u8 != 0 && self.previous_frame & mouse_code as u8 == 0
    }

    pub fn held(&self, mouse_code: MOUSECODE) -> bool {
        self.current_frame & mouse_code as u8 != 0
    }

    pub fn released(&self, mouse_code: MOUSECODE) -> bool {
        self.current_frame & mouse_code as u8 == 0 && self.previous_frame & mouse_code as u8 != 0
    }

    pub fn advance_input(&mut self) {
        self.previous_frame = self.current_frame;
        self.wheel_delta = 0;
        self.wheel_h_delta = 0;
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum GAMEPAD {
    A = XINPUT_GAMEPAD_A,
    B = XINPUT_GAMEPAD_B,
    X = XINPUT_GAMEPAD_X,
    Y = XINPUT_GAMEPAD_Y,
    UP = XINPUT_GAMEPAD_DPAD_UP,
    DOWN = XINPUT_GAMEPAD_DPAD_DOWN,
    LEFT = XINPUT_GAMEPAD_DPAD_LEFT,
    RIGHT = XINPUT_GAMEPAD_DPAD_RIGHT,
    LSHOULDER = XINPUT_GAMEPAD_LEFT_SHOULDER,
    RSHOULDER = XINPUT_GAMEPAD_RIGHT_SHOULDER,
    LTHUMB = XINPUT_GAMEPAD_LEFT_THUMB,
    RTHUMB = XINPUT_GAMEPAD_RIGHT_THUMB,
    BACK = XINPUT_GAMEPAD_BACK,
    START = XINPUT_GAMEPAD_START,
}

pub struct Controller {
    prev_frame: XINPUT_STATE,
    current_frame: XINPUT_STATE,
    vibration: XINPUT_VIBRATION,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            prev_frame: XINPUT_STATE::default(),
            current_frame: XINPUT_STATE::default(),
            vibration: XINPUT_VIBRATION::default(),
        }
    }
    pub fn process_input(&mut self, controller_state: &XINPUT_STATE) {
        std::mem::swap(&mut self.current_frame, &mut self.prev_frame);
        self.current_frame = controller_state.clone();
    }

    pub fn set_vibration(&mut self, left_speed: u16, right_speed: u16) {
        self.vibration.wLeftMotorSpeed = left_speed;
        self.vibration.wRightMotorSpeed = right_speed;
    }

    pub fn pressed(&self, gamepad: GAMEPAD) -> bool {
        self.current_frame.Gamepad.wButtons & gamepad as u16 != 0
            && self.prev_frame.Gamepad.wButtons & gamepad as u16 == 0
    }

    pub fn held(&self, gamepad: GAMEPAD) -> bool {
        self.current_frame.Gamepad.wButtons & gamepad as u16 != 0
    }

    pub fn released(&self, gamepad: GAMEPAD) -> bool {
        self.current_frame.Gamepad.wButtons & gamepad as u16 == 0
            && self.prev_frame.Gamepad.wButtons & gamepad as u16 != 0
    }

    pub fn left_trigger_pressed(&self, threshold: u8) -> bool {
        self.current_frame.Gamepad.bLeftTrigger >= threshold
            && self.prev_frame.Gamepad.bLeftTrigger < threshold
    }

    pub fn left_trigger_held(&self, threshold: u8) -> bool {
        self.current_frame.Gamepad.bLeftTrigger >= threshold
    }

    pub fn left_trigger_released(&self, threshold: u8) -> bool {
        self.current_frame.Gamepad.bLeftTrigger < threshold
            && self.prev_frame.Gamepad.bLeftTrigger >= threshold
    }

    pub fn get_left_trigger(&self) -> u8 {
        self.current_frame.Gamepad.bLeftTrigger
    }

    pub fn right_trigger_pressed(&self, threshold: u8) -> bool {
        self.current_frame.Gamepad.bRightTrigger >= threshold
            && self.prev_frame.Gamepad.bRightTrigger < threshold
    }

    pub fn right_trigger_held(&self, threshold: u8) -> bool {
        self.current_frame.Gamepad.bRightTrigger >= threshold
    }

    pub fn right_trigger_released(&self, threshold: u8) -> bool {
        self.current_frame.Gamepad.bRightTrigger < threshold
            && self.prev_frame.Gamepad.bRightTrigger >= threshold
    }

    pub fn get_right_trigger(&self) -> u8 {
        self.current_frame.Gamepad.bRightTrigger
    }

    pub fn get_left_stick_x(&self) -> i16 {
        self.current_frame.Gamepad.sThumbLX
    }

    pub fn get_left_stick_y(&self) -> i16 {
        self.current_frame.Gamepad.sThumbLY
    }

    pub fn get_right_stick_x(&self) -> i16 {
        self.current_frame.Gamepad.sThumbRX
    }

    pub fn get_right_stick_y(&self) -> i16 {
        self.current_frame.Gamepad.sThumbRY
    }
}

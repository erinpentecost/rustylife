use fixedbitset::FixedBitSet;
use num_enum::TryFromPrimitive;
use std::fmt;
use strum_macros::AsRefStr;

// https://doc.rust-lang.org/std/fmt/trait.Display.html

// This is basically a direct map of HTML5 input events.
// If a key is held down, it will continue to spawn events.
// It'd be nice to have a quantizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputEvent {
    // Mouse IDs: Hover = 230, Left = 231, Middle = 232, Right = 233.
    // These are HTML mouse events offset by + 230
    // Upper left corner is 0, 0
    MouseEvent {
        id: u8,
        pressed: bool,
        x: i32,
        y: i32,
    },
    // Key IDs: http://gcctech.org/csc/javascript/javascript_keycodes.htm
    // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values
    KeyEvent {
        id: u8,
        pressed: bool,
    },
}

impl fmt::Display for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputEvent::MouseEvent { id, pressed, x, y } => {
                write!(f, "MouseEvent({}, {}, {}, {})", id, pressed, x, y)
            }
            InputEvent::KeyEvent { id, pressed } => write!(f, "KeyEvent({}, {})", id, pressed),
        }
    }
}

const KEY_COUNT: usize = 256;

// A snapshot of inputs after they've been massaged.
// I don't really expect more than a few keys to be pressed
// at a time, so a beefy hashmap probably isn't going to beat
// a list of tuples.
// Ok, so my frames per second isn't the best (40ish).
// This causes a problem on input tracking when buttons
// can be released and pressed again on the same frame.
// To resolve this, a simplification is made:
// If you press a button on a frame but do not release -> pressed
// If you release a button that was pressed on a frame before -> released
// If you release but then press again on the same frame -> pressed
// If you press a button and release on the same frame -> pressed
pub struct InputMap {
    // Keys that are currently held down. They may have been first pressed
    // down on a previous frame.
    held_keys: FixedBitSet,
    // Keys that have been newly pressed down.
    hit_keys: FixedBitSet,
    delayed_key_ups: FixedBitSet,
    pub cursor_x: i32,
    pub cursor_y: i32,
}

impl InputMap {
    pub fn new() -> InputMap {
        let held_keys = FixedBitSet::with_capacity(KEY_COUNT);
        let hit_keys = FixedBitSet::with_capacity(KEY_COUNT);
        let delayed_key_ups = FixedBitSet::with_capacity(KEY_COUNT);
        InputMap {
            held_keys,
            hit_keys,
            delayed_key_ups,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// True if the key is currently being held down.
    /// The key may have been initially pressed down on a previous frame.
    pub fn held(&self, key: Key) -> bool {
        self.held_keys.contains(key as usize)
    }

    /// True if the key is held down AND it was not held down on the
    /// previous frame.
    pub fn hit(&self, key: Key) -> bool {
        self.hit_keys.contains(key as usize)
    }

    pub fn clear_hits(&mut self) {
        self.hit_keys.clear();
    }

    pub fn process_new_events(&mut self, events: &[InputEvent]) {
        // First, process delayed key ups.
        for i in 0..KEY_COUNT {
            if self.delayed_key_ups.contains(i) {
                self.held_keys.set(i, false);
            }
        }
        self.delayed_key_ups.clear();

        assert_eq!(self.held_keys.len(), KEY_COUNT);
        assert_eq!(self.hit_keys.len(), KEY_COUNT);
        assert_eq!(self.delayed_key_ups.len(), KEY_COUNT);

        let prev_held = self.held_keys.clone();
        // Track if a key goes off at all. Used for very fast clicks
        // that happen -during- a frame.
        let mut pressed_keys_at_all = FixedBitSet::with_capacity(KEY_COUNT);

        // I care about three things:
        // 1. State of previous frame
        // 2. State at the end of this frame
        // 3. If a button was pressed at all this frame when it didn't start as pressed
        for event in events {
            match event {
                InputEvent::KeyEvent { id, pressed } => {
                    self.held_keys.set(*id as usize, *pressed);
                    // Don't lose really fast presses.
                    if *pressed {
                        pressed_keys_at_all.set(*id as usize, true);
                    }
                }
                InputEvent::MouseEvent { id, pressed, x, y } => {
                    self.held_keys.set(*id as usize, *pressed);
                    // Don't lose really fast presses.
                    if *pressed {
                        pressed_keys_at_all.set(*id as usize, true);
                    }
                    self.cursor_x = *x;
                    self.cursor_y = *y;
                }
            }
        }
        // Track which are pressed
        for i in 0..KEY_COUNT {
            if !prev_held.contains(i)
                && !self.held_keys.contains(i)
                && pressed_keys_at_all.contains(i)
            {
                // Pressed and released on same frame? Don't lose it!
                self.held_keys.set(i, true);
                self.hit_keys.set(i, true);
                // If the key wasn't depressed at the end of this frame, this will cause it be
                // "stuck down" even though it isn't. The fix is to add it to the delayed_down list.
                self.delayed_key_ups.set(i, true);
            } else {
                // Track diffs in hit set.
                self.hit_keys
                    .set(i, !prev_held.contains(i) && self.held_keys.contains(i));
            }
        }

        // Special case for mouse hover.
        self.held_keys.set(Key::MouseHover as usize, false);
        self.hit_keys.set(Key::MouseHover as usize, false);
    }
}

/// This is based on the HTML5 key code identifiers.
/// 230+ have been reserved for mouse keys.
#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr, TryFromPrimitive, EnumIter)]
pub enum Key {
    Backspace = 8,
    Tab = 9,
    Enter = 13,
    Shift = 16,
    Ctrl = 17,
    Alt = 18,
    Pause = 19,
    CapsLock = 20,
    Escape = 27,
    PageUp = 33,
    Space = 32,
    PageDown = 34,
    End = 35,
    Home = 36,
    ArrowLeft = 37,
    ArrowUp = 38,
    ArrowRight = 39,
    ArrowDown = 40,
    PrintScreen = 44,
    Insert = 45,
    Delete = 46,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
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
    LeftWindowKey = 91,
    RightWindowKey = 92,
    SelectKey = 93,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    Multiply = 106,
    Add = 107,
    Subtract = 109,
    DecimalPoint = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    NumLock = 144,
    ScrollLock = 145,
    MyComputer = 182,
    MyCalculator = 183,
    Semicolon = 186,
    EqualSign = 187,
    Comma = 188,
    Dash = 189,
    Period = 190,
    ForwardSlash = 191,
    OpenBracket = 219,
    BackSlash = 220,
    CloseBraket = 221,
    SingleQuote = 222,
    /// This isn't an actual key. This means the cursor
    /// was moved while no mouse button was pressed.
    MouseHover = 230,
    MouseLeft = 231,
    MouseMiddle = 232,
    MouseRight = 233,
}

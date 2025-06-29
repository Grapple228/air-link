#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Letter {
    ///	"A" key.
    A = 0x41,
    ///	"B" key.
    B = 0x42,
    ///	"C" key.
    C = 0x43,
    ///	"D" key.
    D = 0x44,
    ///	"E" key.
    E = 0x45,
    ///	"F" key.
    F = 0x46,
    ///	"G" key.
    G = 0x47,
    ///	"H" key.
    H = 0x48,
    ///	"I" key.
    I = 0x49,
    ///	"J" key.
    J = 0x4A,
    ///	"K" key.
    K = 0x4B,
    ///	"L" key.
    L = 0x4C,
    ///	"M" key.
    M = 0x4D,
    ///	"N" key.
    N = 0x4E,
    ///	"O" key.
    O = 0x4F,
    ///	"P" key.
    P = 0x50,
    ///	"Q" key.
    Q = 0x51,
    ///	"R" key.
    R = 0x52,
    ///	"S" key.
    S = 0x53,
    ///	"T" key.
    T = 0x54,
    ///	"U" key.
    U = 0x55,
    ///	"V" key.
    V = 0x56,
    ///	"W" key.
    W = 0x57,
    ///	"X" key.
    X = 0x58,
    ///	"Y" key.
    Y = 0x59,
    ///	"Z" key.
    Z = 0x5A,
}

impl Letter {
    pub fn from_u8(value: u8) -> Option<Letter> {
        match value {
            0x41 => Some(Letter::A),
            0x42 => Some(Letter::B),
            0x43 => Some(Letter::C),
            0x44 => Some(Letter::D),
            0x45 => Some(Letter::E),
            0x46 => Some(Letter::F),
            0x47 => Some(Letter::G),
            0x48 => Some(Letter::H),
            0x49 => Some(Letter::I),
            0x4A => Some(Letter::J),
            0x4B => Some(Letter::K),
            0x4C => Some(Letter::L),
            0x4D => Some(Letter::M),
            0x4E => Some(Letter::N),
            0x4F => Some(Letter::O),
            0x50 => Some(Letter::P),
            0x51 => Some(Letter::Q),
            0x52 => Some(Letter::R),
            0x53 => Some(Letter::S),
            0x54 => Some(Letter::T),
            0x55 => Some(Letter::U),
            0x56 => Some(Letter::V),
            0x57 => Some(Letter::W),
            0x58 => Some(Letter::X),
            0x59 => Some(Letter::Y),
            0x5A => Some(Letter::Z),
            _ => None, // Возвращаем None, если значение не соответствует ни одному из вариантов
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Number {
    ///	"0" key in standard key location.
    Zero = 0x30,
    ///	"1" key in standard key location.
    One = 0x31,
    ///	"2" key in standard key location.
    Two = 0x32,
    ///	"3" key in standard key location.
    Three = 0x33,
    ///	"4" key in standard key location.
    Four = 0x34,
    ///	"5" key in standard key location.
    Five = 0x35,
    ///	"6" key in standard key location.
    Six = 0x36,
    ///	"7" key in standard key location.
    Seven = 0x37,
    ///	"8" key in standard key location.
    Eight = 0x38,
    ///	"9" key in standard key location.
    Nine = 0x39,
}

impl Number {
    pub fn from_u8(value: u8) -> Option<Number> {
        match value {
            0x30 => Some(Number::Zero),
            0x31 => Some(Number::One),
            0x32 => Some(Number::Two),
            0x33 => Some(Number::Three),
            0x34 => Some(Number::Four),
            0x35 => Some(Number::Five),
            0x36 => Some(Number::Six),
            0x37 => Some(Number::Seven),
            0x38 => Some(Number::Eight),
            0x39 => Some(Number::Nine),
            _ => None, // Возвращаем None, если значение не соответствует ни одному из вариантов
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Numpad {
    ///	"0" on the numeric keypad.
    NUMPAD0 = 0x60,
    ///	"1" on the numeric keypad.
    NUMPAD1 = 0x61,
    ///	"2" on the numeric keypad.
    NUMPAD2 = 0x62,
    ///	"3" on the numeric keypad.
    NUMPAD3 = 0x63,
    ///	"4" on the numeric keypad.
    NUMPAD4 = 0x64,
    ///	"5" on the numeric keypad.
    NUMPAD5 = 0x65,
    ///	"6" on the numeric keypad.
    NUMPAD6 = 0x66,
    ///	"7" on the numeric keypad.
    NUMPAD7 = 0x67,
    ///	"8" on the numeric keypad.
    NUMPAD8 = 0x68,
    ///	"9" on the numeric keypad.
    NUMPAD9 = 0x69,
    ///	"*" on the numeric keypad.
    MULTIPLY = 0x6A,
    ///	"+" on the numeric keypad.
    ADD = 0x6B,
    ///	"-" on the numeric keypad.
    SUBTRACT = 0x6D,
    ///	Decimal point on the numeric keypad.
    DECIMAL = 0x6E,
    ///	"/" on the numeric keypad.
    DIVIDE = 0x6F,
    ///	Num Lock key.
    NUM_LOCK = 0x90,
}

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum F {
    ///	F1 key.
    F1 = 0x70,
    ///	F2 key.
    F2 = 0x71,
    ///	F3 key.
    F3 = 0x72,
    ///	F4 key.
    F4 = 0x73,
    ///	F5 key.
    F5 = 0x74,
    ///	F6 key.
    F6 = 0x75,
    ///	F7 key.
    F7 = 0x76,
    ///	F8 key.
    F8 = 0x77,
    ///	F9 key.
    F9 = 0x78,
    ///	F10 key.
    F10 = 0x79,
    ///	F11 key.
    F11 = 0x7A,
    ///	F12 key.
    F12 = 0x7B,
    ///	F13 key.
    F13 = 0x7C,
    ///	F14 key.
    F14 = 0x7D,
    ///	F15 key.
    F15 = 0x7E,
    ///	F16 key.
    F16 = 0x7F,
    ///	F17 key.
    F17 = 0x80,
    ///	F18 key.
    F18 = 0x81,
    ///	F19 key.
    F19 = 0x82,
    ///	F20 key.
    F20 = 0x83,
    ///	F21 key.
    F21 = 0x84,
    ///	F22 key.
    F22 = 0x85,
    ///	F23 key.
    F23 = 0x86,
    ///	F24 key.
    F24 = 0x87,
}

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum KeyboardButton {
    ///	Cancel key.
    CANCEL = 0x03,
    ///	Help key.
    HELP = 0x06,
    ///	Backspace key.
    BACK_SPACE = 0x08,
    ///	Tab key.
    TAB = 0x09,
    ///	"5" key on Numpad when NumLock is unlocked. Or on Mac, clear key which is positioned at NumLock key.
    CLEAR = 0x0C,
    ///	Return/enter key on the main keyboard.
    RETURN = 0x0D,
    ENTER = 0x0E,
    ///	Shift key.
    SHIFT = 0x10,
    ///	Control key.
    CONTROL = 0x11,
    ///	Alt (Option on Mac) key.
    ALT = 0x12,
    ///	Pause key.
    PAUSE = 0x13,
    ///	Caps lock.
    CAPS_LOCK = 0x14,
    ///	Escape key.
    ESCAPE = 0x1B,
    CONVERT = 0x1C,
    NONCONVERT = 0x1D,
    ACCEPT = 0x1E,
    MODECHANGE = 0x1F,
    ///	Space bar.
    SPACE = 0x20,
    ///	Page Up key.
    PAGE_UP = 0x21,
    ///	Page Down key.
    PAGE_DOWN = 0x22,
    ///	End key.
    END = 0x23,
    ///	Home key.
    HOME = 0x24,
    ///	Left arrow.
    LEFT = 0x25,
    ///	Up arrow.
    UP = 0x26,
    ///	Right arrow.
    RIGHT = 0x27,
    ///	Down arrow.
    DOWN = 0x28,
    SELECT = 0x29,
    PRINT = 0x2A,
    EXECUTE = 0x2B,
    ///	Print Screen key.
    PRINTSCREEN = 0x2C,
    ///	Ins(ert) key.
    INSERT = 0x2D,
    ///	Del(ete) key.
    DELETE = 0x2E,
    ///	Colon (":") key.
    COLON = 0x3A,
    ///	Semicolon (",") key.
    SEMICOLON = 0x3B,
    ///	Less-than ("<") key.
    LESS_THAN = 0x3C,
    ///	Equals ("=") key.
    EQUALS = 0x3D,
    ///	Greater-than (">") key.
    GREATER_THAN = 0x3E,
    ///	Question mark ("?") key.
    QUESTION_MARK = 0x3F,
    ///	Atmark ("@") key.
    AT = 0x40,
    ///	Windows logo key on Windows. Or Super or Hyper key on Linux.
    WIN = 0x5B,
    ///	Opening context menu key.
    CONTEXT_MENU = 0x5D,
    SLEEP = 0x5F,
    SEPARATOR = 0x6C,

    ///	Scroll Lock key.
    SCROLL_LOCK = 0x91,
    ///	Circumflex ("^") key.
    CIRCUMFLEX = 0xA0,
    ///	Exclamation ("!") key.
    EXCLAMATION = 0xA1,
    ///	Hash ("#") key.
    HASH = 0xA3,
    ///	Dollar sign ("$") key.
    DOLLAR = 0xA4,
    ///	Percent ("%") key.
    PERCENT = 0xA5,
    ///	Ampersand ("&") key.
    AMPERSAND = 0xA6,
    ///	Underscore ("_") key.
    UNDERSCORE = 0xA7,
    ///	Open parenthesis ("(") key.
    OPEN_PAREN = 0xA8,
    ///	Close parenthesis (")") key.
    CLOSE_PAREN = 0xA9,
    ///	Asterisk ("*") key.
    ASTERISK = 0xAA,
    ///	Plus ("+") key.
    PLUS = 0xAB,
    ///	Pipe ("|") key.
    PIPE = 0xAC,
    ///	Hyphen-US/docs/Minus ("-") key.
    HYPHEN_MINUS = 0xAD,
    ///	Open curly bracket ("{") key.
    OPEN_CURLY_BRACKET = 0xAE,
    ///	Close curly bracket ("}") key.
    CLOSE_CURLY_BRACKET = 0xAF,
    ///	Tilde ("~") key.
    TILDE = 0xB0,
    ///	Audio mute key.
    VOLUME_MUTE = 0xB5,
    ///	Audio volume down key
    VOLUME_DOWN = 0xB6,
    ///	Audio volume up key
    VOLUME_UP = 0xB7,
    ///	Comma (",") key.
    COMMA = 0xBC,
    ///	Period (".") key.
    PERIOD = 0xBE,
    ///	Slash ("/") key.
    SLASH = 0xBF,
    ///	Back tick ("`") key.
    BACK_QUOTE = 0xC0,
    ///	Open square bracket ("[") key.
    OPEN_BRACKET = 0xDB,
    ///	Back slash ("\") key.
    BACK_SLASH = 0xDC,
    ///	Close square bracket ("]") key.
    CLOSE_BRACKET = 0xDD,
    ///	Quote (''') key.
    QUOTE = 0xDE,
    ///	Meta key on Linux, Command key on Mac.
    META = 0xE0,
    ///	AltGr key (Level 3 Shift key or Level 5 Shift key) on Linux.
    ALTGR = 0xE1,
    ///	Attn (Attention) key of IBM midrange computers, e.g., AS/400.
    ATTN = 0xF6,
    ///	CrSel (Cursor Selection) key of IBM 3270 keyboard layout.
    CRSEL = 0xF7,
    ///	ExSel (Extend Selection) key of IBM 3270 keyboard layout.
    EXSEL = 0xF8,
    ///	Erase EOF key of IBM 3270 keyboard layout.
    EREOF = 0xF9,
    ///	Play key of IBM 3270 keyboard layout.
    PLAY = 0xFA,
    ///	Zoom key.
    ZOOM = 0xFB,
    ///	PA1 key of IBM 3270 keyboard layout.
    PA1 = 0xFD,
    /// Clear key, but we're not sure the meaning difference from CLEAR.
    WIN_OEM_CLEAR = 0xFE,

    Letter(Letter),
    Number(Number) = 0,
    Numpad(Numpad),
    F(F),
}

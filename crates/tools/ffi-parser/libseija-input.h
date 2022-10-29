

enum KeyCode {
  /**
   * The '1' key over the letters.
   */
  Key1,
  /**
   * The '2' key over the letters.
   */
  Key2,
  /**
   * The '3' key over the letters.
   */
  Key3,
  /**
   * The '4' key over the letters.
   */
  Key4,
  /**
   * The '5' key over the letters.
   */
  Key5,
  /**
   * The '6' key over the letters.
   */
  Key6,
  /**
   * The '7' key over the letters.
   */
  Key7,
  /**
   * The '8' key over the letters.
   */
  Key8,
  /**
   * The '9' key over the letters.
   */
  Key9,
  /**
   * The '0' key over the 'O' and 'P' keys.
   */
  Key0,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  /**
   * The Escape key, next to F1.
   */
  Escape,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  /**
   * Print Screen/SysRq.
   */
  Snapshot,
  /**
   * Scroll Lock.
   */
  Scroll,
  /**
   * Pause/Break key, next to Scroll lock.
   */
  Pause,
  /**
   * `Insert`, next to Backspace.
   */
  Insert,
  Home,
  Delete,
  End,
  PageDown,
  PageUp,
  Left,
  Up,
  Right,
  Down,
  Backspace,
  /**
   * The Enter key.
   */
  Return,
  /**
   * The space bar.
   */
  Space,
  /**
   * The "Compose" key on Linux.
   */
  Compose,
  Caret,
  Numlock,
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  NumpadAdd,
  NumpadDivide,
  NumpadDecimal,
  NumpadComma,
  NumpadEnter,
  NumpadEquals,
  NumpadMultiply,
  NumpadSubtract,
  AbntC1,
  AbntC2,
  Apostrophe,
  Apps,
  Asterisk,
  At,
  Ax,
  Backslash,
  Calculator,
  Capital,
  Colon,
  Comma,
  Convert,
  Equals,
  Grave,
  Kana,
  Kanji,
  LAlt,
  LBracket,
  LControl,
  LShift,
  LWin,
  Mail,
  MediaSelect,
  MediaStop,
  Minus,
  Mute,
  MyComputer,
  NavigateForward,
  NavigateBackward,
  NextTrack,
  NoConvert,
  OEM102,
  Period,
  PlayPause,
  Plus,
  Power,
  PrevTrack,
  RAlt,
  RBracket,
  RControl,
  RShift,
  RWin,
  Semicolon,
  Slash,
  Sleep,
  Stop,
  Sysrq,
  Tab,
  Underline,
  Unlabeled,
  VolumeDown,
  VolumeUp,
  Wake,
  WebBack,
  WebFavorites,
  WebForward,
  WebHome,
  WebRefresh,
  WebSearch,
  WebStop,
  Yen,
  Copy,
  Paste,
  Cut,
  Unknow,
};
typedef uint32_t KeyCode;

typedef struct Input Input;

void input_add_module(App *app_ptr);

bool input_get_keydown(const struct Input *input, KeyCode keycode);

bool input_get_keyup(const struct Input *input, KeyCode keycode);

bool input_get_mouse_down(const struct Input *input, uint32_t mouse_btn);

bool input_get_mouse_up(const struct Input *input, uint32_t mouse_btn);

const struct Input *input_world_get_input(const World *world);

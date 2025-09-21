use std::collections::HashMap;
use winit::{
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
    platform::scancode::PhysicalKeyExtScancode,
};

use crate::core::application::input::types::KeyInput;

pub mod types;

static mut KEYS_DOWN: Option<HashMap<PhysicalKey, KeyEvent>> = None;

#[allow(dead_code)]
pub fn get_input(event: &KeyEvent) -> KeyInput {
    let key: String = get_input_key(event);
    let pressed: bool = event.state.is_pressed();
    let held_keys: Vec<String>;

    unsafe {
        #[allow(static_mut_refs)]
        let keys = KEYS_DOWN.get_or_insert_with(HashMap::new);

        if pressed {
            keys.insert(event.physical_key.clone(), event.clone());
        } else {
            keys.remove(&event.physical_key);
        }

        held_keys = keys
            .iter()
            .filter(|(k, _)| *k != &event.physical_key)
            .map(|(_, e)| get_input_key(e))
            .collect();
    }

    KeyInput {
        key,
        pressed,
        held_keys,
    }
}

pub fn get_input_key(event: &KeyEvent) -> String {
    format!("{:?}", event.physical_key.to_scancode())
}

#[allow(dead_code)]
pub fn get_input_key_pretty(event: &KeyEvent) -> String {
    use KeyCode::*;
    let key = format!("{:?}", event.physical_key.to_scancode());

    match event.physical_key {
        PhysicalKey::Code(code) => match code {
            ControlLeft => "lctrl",
            ControlRight => "rctrl",

            ShiftLeft => "lshift",
            ShiftRight => "rshift",

            AltLeft => "lalt",
            AltRight => "ralt",

            ArrowUp => "up",
            ArrowRight => "right",
            ArrowDown => "down",
            ArrowLeft => "left",

            CapsLock => "capslock",
            Tab => "tab",
            Escape => "escape",

            Fn => "function",
            Enter => "Enter",
            Backspace => "backspace",

            Insert => "insert",
            _ => event.logical_key.to_text().unwrap_or(&key),
        },

        _ => event.logical_key.to_text().unwrap_or(&key),
    }
    .to_string()
}

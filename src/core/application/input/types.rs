#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyInput {
    pub input: KeyInputButton,
    pub pressed: bool,
    pub held_keys: Vec<KeyInputButton>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyInputButton {
    pub key: String,
    pub code: String,
}

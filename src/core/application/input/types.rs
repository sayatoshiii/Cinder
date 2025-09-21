#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyInput {
    pub input: KeyInputButton,
    pub pressed: bool,
    pub held_keys: Vec<KeyInputButton>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyInputButton {
    pub key: String,
    pub code: String,
}

use super::moves::Move;

pub struct Pokemon {
    pub species: String,
    pub level: u8,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub current_hp: u8,
    pub moves: [Move; 4],
}

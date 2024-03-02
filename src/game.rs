#[derive(Debug)]
pub struct GameController {
    pub players: Vec<Player>,
}

impl GameController {
    pub fn new() -> Self {
        let player = Player::new([0.0, 0.0]);
        Self {
            players: vec![player],
        }
    }
}

#[derive(Debug)]
pub struct Player {
    position: [f32; 2],
    size: [f32; 2],
    velocity: f32,
}

impl Player {
    pub fn new(position: [f32; 2]) -> Self {
        Self {
            position,
            size: [0.2, 0.2],
            velocity: 0.5,
        }
    }

    pub fn position(&self) -> &[f32; 2] {
        &self.position
    }

    pub fn size(&self) -> &[f32; 2] {
        &self.size
    }
}

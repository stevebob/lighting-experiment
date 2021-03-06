use content::{DoorType, TileSprite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoorState {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorInfo {
    pub typ: DoorType,
    pub state: DoorState,
}

impl DoorInfo {
    pub fn new(typ: DoorType, state: DoorState) -> Self {
        DoorInfo { typ, state }
    }

    pub fn sprite(self) -> TileSprite {
        self.typ.state_sprite(self.state)
    }
}

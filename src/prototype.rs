use cgmath::Vector2;
use entity_store::{EntityId, EntityChange, insert};
use content::{TileSprite, DepthType, DepthInfo, DoorState, DoorInfo,
              DoorType, SpriteEffectInfo, LightInfo, HealthInfo,
              FieldUiOffsets};
use append::Append;

pub fn angler<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::Angler));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.39)));
    changes.append(insert::collider(id));
    changes.append(insert::player(id));
    changes.append(insert::door_opener(id));
    changes.append(insert::light(id, LightInfo::new(0.2, 20, 1.0, 1.0, 1.0, 1.0)));
    changes.append(insert::bump_attack(id));
    changes.append(insert::attackable(id));
}

pub fn crab<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::Crab));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.4)));
    changes.append(insert::collider(id));
    changes.append(insert::npc(id));
    changes.append(insert::bump_attack(id));
    changes.append(insert::attackable(id));
    changes.append(insert::health(id, HealthInfo::full(8)));
    changes.append(insert::field_ui(id, FieldUiOffsets {
        health_vertical: 7,
    }));
    changes.append(insert::hide_in_dark(id));
}

pub fn snail<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::Snail));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.4)));
    changes.append(insert::collider(id));
    changes.append(insert::npc(id));
    changes.append(insert::bump_attack(id));
    changes.append(insert::attackable(id));
    changes.append(insert::health(id, HealthInfo::full(3)));
    changes.append(insert::field_ui(id, FieldUiOffsets {
        health_vertical: 1,
    }));
    changes.append(insert::hide_in_dark(id));
}

pub fn inner_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, TileSprite::InnerWall));
    changes.append(insert::depth(id, DepthType::Fixed.into()));
}

pub fn outer_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, TileSprite::OuterWall));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.75)));
}

pub fn inner_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::InnerFloor));
    changes.append(insert::depth(id, DepthType::Bottom.into()));
}

pub fn inner_water<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::InnerWater));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Gradient, 0.01)));
    changes.append(insert::sprite_effect(id, SpriteEffectInfo::water(6, 0.3, 0.7)));
}

pub fn outer_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::OuterFloor));
    changes.append(insert::depth(id, DepthType::Bottom.into()));
    changes.append(insert::sprite_effect(id, SpriteEffectInfo::water(3, 0.2, 0.8)));
}

pub fn inner_door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::InnerDoor));
    changes.append(insert::depth(id, DepthType::Gradient.into()));
    changes.append(insert::door(id, DoorInfo::new(DoorType::Inner, DoorState::Closed)));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
}

pub fn outer_door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::OuterDoor));
    changes.append(insert::depth(id, DepthType::Gradient.into()));
    changes.append(insert::door(id, DoorInfo::new(DoorType::Outer, DoorState::Closed)));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
}

pub fn window<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::Window));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, 0.5)));
    changes.append(insert::opacity(id, -1.0));
}

pub fn light<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>, colour: [f32; 3]) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::position(id, coord.cast()));
    changes.append(insert::sprite(id, TileSprite::Light));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, 0.0)));
    changes.append(insert::light(id, LightInfo::new(1.0, 20, 2.0, colour[0], colour[1], colour[2])));
}

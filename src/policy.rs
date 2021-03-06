use std::time::Duration;
use entity_store::{EntityId, EntityChange, ComponentValue, EntityStore, insert, remove};
use spatial_hash::SpatialHashTable;
use append::Append;
use content::{ChangeDesc, DoorState};

pub fn check<R, D>(change: &EntityChange,
                   entity_store: &EntityStore,
                   spatial_hash: &SpatialHashTable,
                   reactions: &mut R,
                   to_delete: &mut D) -> bool
    where R: Append<ChangeDesc>,
          D: Append<EntityId>,
{
    use self::EntityChange::*;
    match change {
        &Insert(id, ComponentValue::Coord(coord)) => {
            if let Some(sh_cell) = spatial_hash.get_signed(coord) {

                if entity_store.door_opener.contains(&id) {
                    // open doors by bumping into them
                    if let Some(door_id) = sh_cell.door_set.iter().next() {
                        if let Some(mut door_info) = entity_store.door.get(door_id).cloned() {
                            if door_info.state == DoorState::Closed {
                                door_info.state = DoorState::Open;
                                reactions.append(ChangeDesc::immediate(insert::door(*door_id, door_info)));
                                return false;
                            }
                        }
                    }
                }

                if entity_store.collider.contains(&id) {
                    // prevent walking into solid cells
                    if sh_cell.solid_count > 0 {
                        return false;
                    }
                }

                if let Some(current_coord) = entity_store.coord.get(&id) {
                    if coord != *current_coord {
                        if entity_store.npc.contains(&id) && sh_cell.npc_count > 0 {
                            return false;
                        }

                        if entity_store.bump_attack.contains(&id) {
                            if let Some(attackable_id) = sh_cell.attackable_set.iter().next() {
                                let mid_change = entity_store.health.get(attackable_id).map(|health| {
                                    insert::health(*attackable_id, health.reduce(1))
                                });
                                reactions.append(ChangeDesc::bump_slide(id,
                                                                        current_coord.cast(),
                                                                        coord.cast(),
                                                                        Duration::from_millis(100),
                                                                        0.49,
                                                                        mid_change));
                                return false;
                            }
                        }

                        // Start the slide animation for the move.
                        reactions.append(ChangeDesc::slide(id, current_coord.cast(), coord.cast(), Duration::from_millis(50)));
                    }
                }
            } else {
                // No spatial hash cell for destination - prevent the move.
                return false;
            }
        }
        &Insert(id, ComponentValue::Door(door_info)) => {
            // animations and status effects of a door being opened or closed
            match door_info.state {
                DoorState::Open => {
                    reactions.append(ChangeDesc::immediate(remove::solid(id)));
                    reactions.append(ChangeDesc::immediate(insert::opacity(id, 0.0)));
                    reactions.append(ChangeDesc::sprites(id, door_info.typ.open_animation(),
                                                         insert::sprite(id, door_info.typ.open_sprite())));
                }
                DoorState::Closed => {
                    if let Some(coord) = entity_store.coord.get(&id) {
                        if let Some(sh_cell) = spatial_hash.get_signed(*coord) {
                            if sh_cell.npc_count > 0 || sh_cell.player_count > 0 {
                                return false;
                            }
                        }
                    }
                    reactions.append(ChangeDesc::immediate(insert::solid(id)));
                    reactions.append(ChangeDesc::sprites(id, door_info.typ.close_animation(),
                                     insert::door_closing_finished(id)));
                }
            }
        }
        &Insert(id, ComponentValue::DoorClosingFinished) => {
            // extra changes to apply once a door is fully closed
            reactions.append(ChangeDesc::immediate(insert::opacity(id, 1.0)));
            if let Some(door_info) = entity_store.door.get(&id) {
                reactions.append(ChangeDesc::immediate(insert::sprite(id, door_info.typ.closed_sprite())));
            }
        }
        &Insert(id, ComponentValue::Health(info)) => {
            if info.current <= 0 {
                to_delete.append(id);
                return false;
            }
        }
        _ => {}
    }

    true
}

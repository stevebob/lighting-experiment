use std::time::Duration;
use cgmath::Vector2;

use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;
use vision::VisionGrid;

use input::Input;

#[derive(Debug, Clone, Copy)]
pub struct VisibleRange {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

impl Default for VisibleRange {
    fn default() -> Self {
        Self {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }
}

pub trait LightUpdate {
    fn set_position(&mut self, position: Vector2<f32>);
    fn set_height(&mut self, height: f32);
    fn set_colour(&mut self, colour: [f32; 3]);
    fn set_intensity(&mut self, intensity: f32);
}

pub trait OutputWorldState<'a, 'b> {
    type VisionCellGrid: VisionGrid;
    type LightCellGrid: VisionGrid;
    type LightUpdate: LightUpdate;
    fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable);
    fn set_player_position(&mut self, player_position: Vector2<f32>);
    fn set_frame_info(&mut self, frame_count: u64, total_time: Duration);
    fn vision_grid(&'b mut self) -> Self::VisionCellGrid;
    fn next_light(&'b mut self) -> Option<(Self::LightCellGrid, &'b mut Self::LightUpdate)>;
}

pub trait FrontendOutput<'a> {
    type WorldState: for<'b> OutputWorldState<'a, 'b>;
    fn with_world_state<F: FnMut(&mut Self::WorldState)>(&'a mut self, f: F);
    fn draw(&mut self);
    fn handle_resize(&mut self, width: u16, height: u16);
    fn update_world_size(&mut self, width: u32, height: u32);
    fn visible_range(&self) -> VisibleRange;
}

pub trait FrontendInput {
    fn with_input<F: FnMut(Input)>(&mut self, f: F);
}

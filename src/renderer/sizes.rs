pub const MAX_NUM_INSTANCES: usize = 65536;
pub const MAX_NUM_LIGHTS: usize = 32;

pub const MAX_CELL_TABLE_SIZE: usize = 16384;
pub const TBO_VISION_FRAME_COUNT_SIZE: usize = 5; // 40 bit uint
pub const TBO_VISION_BITMAP_SIZE: usize = 1; // 8 bit bitmap
pub const TBO_VISION_BITMAP_OFFSET: usize = TBO_VISION_FRAME_COUNT_SIZE;
pub const TBO_VISION_ENTRY_SIZE: usize = TBO_VISION_FRAME_COUNT_SIZE + TBO_VISION_BITMAP_SIZE;
pub const TBO_VISION_BUFFER_SIZE: usize = TBO_VISION_ENTRY_SIZE * MAX_CELL_TABLE_SIZE;

pub const LIGHT_BUFFER_SIZE: usize = TBO_VISION_BUFFER_SIZE * MAX_NUM_LIGHTS;

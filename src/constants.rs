// Window
// const WW: f32 = 1024.0;
// const WH: f32 = 576.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.5;
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const SPRITE_SHEET_W: u32 = 8;
pub const SPRITE_SHEET_H: u32 = 8;

// World
pub const NUM_WORLD_DECORATIONS: u32 = 4000;
pub const WORLD_W: f32 = 5120.0;
pub const WORLD_H: f32 = 2880.0;

//Player
pub const PLAYER_SPEED: f32 = 2.0;

// Gun
pub const GUN_HEIGHT: f32 = 16.0;
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_SPEED: f32 = 10.0;
pub const BULLET_DAMAGE: f32 = 100.0;
pub const BULLET_TIME_SECS: f32 = 1.0;
pub const NUM_BULLETS_PER_SHOT: usize = 5;

// Enemy
pub const MAX_NUMBER_ENEMY: usize = 100_000;
pub const SPAWN_RATE_PER_SECOND: usize = 2_000;
pub const ENEMY_HEALTH: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (251, 245, 239);

use godot::prelude::*;

mod level;
mod player;
mod player_death;
mod player_camera;
mod bullet;
mod bullet_impact;
mod enemy;
mod enemy_death;
mod game_manager;
mod health_manager;
mod health_bar;
mod game_screen;

struct Metroidvania;

#[gdextension]
unsafe impl ExtensionLibrary for Metroidvania {}

use godot::prelude::*;

mod level;
mod player;
mod bullet;
mod enemy;
mod game_manager;

struct Metroidvania;

#[gdextension]
unsafe impl ExtensionLibrary for Metroidvania {}

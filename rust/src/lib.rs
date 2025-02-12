use godot::prelude::*;

mod level;
mod player;
mod enemy;
mod game_manager;

struct Metroidvania;

#[gdextension]
unsafe impl ExtensionLibrary for Metroidvania {}

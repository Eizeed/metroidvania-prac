use godot::prelude::*;

mod level;
mod player;
mod enemy;

struct Metroidvania;

#[gdextension]
unsafe impl ExtensionLibrary for Metroidvania {}

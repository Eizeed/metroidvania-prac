use godot::prelude::*;

mod level;
mod player;
mod bullet;
mod enemy;
mod ui;
mod script;
mod collectable;

struct Metroidvania;

#[gdextension]
unsafe impl ExtensionLibrary for Metroidvania {}

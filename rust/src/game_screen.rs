use godot::{classes::{CanvasLayer, ICanvasLayer, RenderingServer}, prelude::*};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
struct GameScreen {
    base: Base<CanvasLayer>
}

#[godot_api]
impl ICanvasLayer for GameScreen {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

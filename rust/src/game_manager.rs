use godot::{classes::RenderingServer, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
struct GameManager {
    base: Base<Node>
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {
        godot_print!("LOADED");
        RenderingServer::singleton().set_default_clear_color(Color::from_rgb(0.44, 0.12, 0.53));
    }
}

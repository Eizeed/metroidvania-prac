use godot::{obj::WithBaseField, prelude::*};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct PlayerCamera {
    base: Base<Camera2D>
}

#[godot_api]
impl ICamera2D for PlayerCamera {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let player = self.base().get_parent().unwrap().get_node_or_null("Player");
        if let Some(player) = player {
            let player = player.cast::<Player>();
            self.base_mut().set_global_position(player.get_global_position());
        }
    }
}

use godot::{obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerDeath {
    base: Base<Node2D>
}

#[godot_api]
impl PlayerDeath {
    #[func]
    fn on_timer_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl INode2D for PlayerDeath {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

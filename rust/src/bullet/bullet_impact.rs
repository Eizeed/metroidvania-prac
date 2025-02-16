use godot::{classes::{AnimatedSprite2D, IAnimatedSprite2D}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct BulletImpact {
    base: Base<AnimatedSprite2D>
}

#[godot_api]
impl BulletImpact {
    #[func]
    fn on_timer_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IAnimatedSprite2D for BulletImpact {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

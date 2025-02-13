use godot::{classes::{AnimatedSprite2D, IAnimatedSprite2D}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Bullet {
    pub direction: f32,
    speed: f32,

    base: Base<AnimatedSprite2D>
}

#[godot_api]
impl Bullet {
    #[func]
    fn on_timer_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IAnimatedSprite2D for Bullet {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            direction: 0.0,
            speed: 600.0,
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let direction = self.direction;
        let speed = self.speed;
        self.base_mut().move_local_x(direction * speed * delta as f32);
    }
}

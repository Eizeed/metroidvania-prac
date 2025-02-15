use godot::{classes::{AnimatedSprite2D, Area2D, IAnimatedSprite2D, PhysicsBody2D}, obj::{NewGd, WithBaseField}, prelude::*};

use crate::bullet_impact;

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Bullet {
    pub direction: f32,
    pub damage_amount: i32,
    speed: f32,

    bullet_impact_scene: Gd<PackedScene>,

    base: Base<AnimatedSprite2D>
}

#[godot_api]
impl Bullet {
    #[func]
    pub fn get_damage_amount(&self) -> i32 {
        self.damage_amount
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_hitbox_area_entered(&mut self, _area: Gd<Area2D>) {
        godot_print!("Bullet area entered");
        self.bullet_impact();
    }

    #[func]
    fn on_hitbox_body_entered(&mut self, _body: Gd<Node2D>) {
        godot_print!("Bullet body entered");
        self.bullet_impact();
    }

    #[func]
    fn bullet_impact(&mut self) {
        let mut bullet_impact_instance = self.bullet_impact_scene.instantiate_as::<bullet_impact::BulletImpact>();
        bullet_impact_instance.set_global_position(self.base().get_global_position());
        self.base().get_parent().unwrap().add_child(&bullet_impact_instance);
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IAnimatedSprite2D for Bullet {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            direction: 0.0,
            damage_amount: 1,
            speed: 600.0,
            bullet_impact_scene: PackedScene::new_gd(),
            base
        }
    }

    fn ready(&mut self) {
        self.bullet_impact_scene = load("res://player/bullet_impact.tscn");
    }

    fn physics_process(&mut self, delta: f64) {
        let direction = self.direction;
        let speed = self.speed;
        self.base_mut().move_local_x(direction * speed * delta as f32);
    }
}

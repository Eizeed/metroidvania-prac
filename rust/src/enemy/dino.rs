use godot::{
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, ProjectSettings},
    obj::WithBaseField,
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Dino {
    state: State,
    #[export]
    slow_down_speed: f32,
    #[export]
    speed: f32,
    #[export]
    damage_amount: i32,
    max_speed: f32,

    base: Base<CharacterBody2D>,
}

#[derive(Debug)]
enum State {
    Idle,
    Attack(Gd<Player>),
}

#[godot_api]
impl Dino {
    #[func]
    fn on_attack_area_enter(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("Player") {
            self.state = State::Attack(body.cast::<Player>());
        };
    }

    #[func]
    fn on_attack_area_exit(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("Player") {
            self.state = State::Idle;
        };
    }

    fn process_idle(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        let mut animation_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        velocity = velocity.move_toward(
            Vector2::new(0.0, velocity.y),
            self.slow_down_speed * delta as f32,
        );
        animation_sprite.set_animation("idle");

        animation_sprite.play();
        self.base_mut().set_velocity(velocity);
    }

    fn process_attack(&mut self, delta: f64) {
        if let State::Attack(player) = &self.state {
            let mut animation_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
            let mut velocity = self.base().get_velocity();

            let direction;
            if self.base().get_global_position().x > player.get_global_position().x {
                animation_sprite.set_flip_h(false);
                direction = -1.0;
            } else {
                animation_sprite.set_flip_h(true);
                direction = 1.0;
            }
            
            animation_sprite.set_animation("attack");
            animation_sprite.play();
            
            velocity.x += direction * self.speed * delta as f32;
            velocity.x = velocity.x.clamp(-self.max_speed, self.max_speed);

            self.base_mut().set_velocity(velocity);
        }
        // unreachable!("Error in code")
    }
}

#[godot_api]
impl ICharacterBody2D for Dino {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            state: State::Idle,
            slow_down_speed: 600.0,
            speed: 200.0,
            max_speed: 0.0,
            damage_amount: 1,
            base,
        }
    }

    fn ready(&mut self) {
        self.max_speed = self.speed + 20.0;
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();

        let gravity: f32 = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to();

        if !self.base().is_on_floor() {
            velocity.y += gravity * delta as f32;
            self.base_mut().set_velocity(velocity);
        }

        match &self.state {
            State::Idle => self.process_idle(delta),
            State::Attack(_) => self.process_attack(delta),
        }

        self.base_mut().move_and_slide();
    }
}

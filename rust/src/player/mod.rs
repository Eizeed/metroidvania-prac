pub mod player_camera;
pub mod player_death;

use godot::{
    classes::{
        AnimatedSprite2D, AnimationPlayer, CharacterBody2D, ICharacterBody2D, Marker2D,
        ProjectSettings,
    },
    obj::{NewGd, WithBaseField},
    prelude::*,
};
use player_death::PlayerDeath;

use crate::{
    bullet,
    enemy::{crab::Crab, dino::Dino}, script::health_manager::HealthManager,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    state: State,
    is_jump: bool,

    bullet_scene: Gd<PackedScene>,
    player_death_scene: Gd<PackedScene>,

    #[export]
    speed: f32,
    #[export]
    jump_velocity: f32,
    #[export]
    jump_horizontal_speed: f32,
    #[export]
    max_horizontal_speed: f32,
    #[export]
    max_jump_horizontal_speed: f32,
    #[export]
    slow_down_speed: f32,

    base: Base<CharacterBody2D>,
}

#[derive(PartialEq)]
enum State {
    Idle,
    Run,
    Jump,
    Fall,
    Shoot,
}

#[godot_api]
impl Player {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node>) {
        if body.is_in_group("Enemy") {
            let damage_amount = {
                let class_name = body.get_class();
                match class_name.to_string().as_str() {
                    "Crab" => body.cast::<Crab>().bind().get_damage_amount(),
                    "Dino" => body.cast::<Dino>().bind().get_damage_amount(),
                    _ => unreachable!("Unexisting classname for Enemy group"),
                }
            };

            let mut health_manager = self
                .base()
                .get_tree()
                .unwrap()
                .get_root()
                .unwrap()
                .get_node_as::<HealthManager>("GlobalHealthManager");

            health_manager.bind_mut().decrease_health(damage_amount);

            let mut hit_animation_player = self
                .base()
                .get_node_as::<AnimationPlayer>("HitAnimationPlayer");
            hit_animation_player.set_current_animation("hit");
            hit_animation_player.play();

            let current_health = health_manager.bind().get_current_health();
            if current_health == 0 {
                self.player_death();
            }
        }
    }

    fn player_death(&mut self) {
        let mut player_death_instance = self.player_death_scene.instantiate_as::<PlayerDeath>();
        player_death_instance.set_global_position(self.base().get_global_position());
        self.base()
            .get_parent()
            .unwrap()
            .add_child(&player_death_instance);
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            state: State::Idle,
            is_jump: false,

            bullet_scene: PackedScene::new_gd(),
            player_death_scene: PackedScene::new_gd(),

            speed: 1000.0,
            jump_velocity: -400.0,
            jump_horizontal_speed: 1000.0,
            max_horizontal_speed: 300.0,
            max_jump_horizontal_speed: 300.0,
            slow_down_speed: 3000.0,

            base,
        }
    }

    fn ready(&mut self) {
        self.bullet_scene = load("res://player/bullet.tscn");
        self.player_death_scene = load("res://player/player_death.tscn");
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        let mut animation = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let input = Input::singleton();

        let gravity: f32 = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to();

        if !self.base().is_on_floor() {
            velocity.y += gravity * delta as f32;
        }

        let direction = input.get_axis("move_left", "move_right");
        if direction != 0.0 {
            if direction > 0.0 {
                animation.set_flip_h(false);
            } else {
                animation.set_flip_h(true);
            }

            velocity.x += direction * self.speed * delta as f32;
            velocity.x = velocity
                .x
                .clamp(-self.max_horizontal_speed, self.max_horizontal_speed);

            if self.base().is_on_floor() {
                self.state = State::Run;
                self.is_jump = false;
            }
        } else {
            velocity = velocity.move_toward(
                Vector2::new(0.0, velocity.y),
                self.slow_down_speed * delta as f32,
            );

            if self.base().is_on_floor() {
                self.state = State::Idle;
                self.is_jump = false;
            }
        }

        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            velocity.y = self.jump_velocity;
            self.state = State::Jump;
            self.is_jump = true;
        }

        if !self.base().is_on_floor() && self.state == State::Jump {
            velocity.x += direction * self.jump_horizontal_speed * delta as f32;
            velocity.x = velocity.x.clamp(
                -self.max_jump_horizontal_speed,
                self.max_jump_horizontal_speed,
            );
        }

        if velocity.y > 0.0 && !self.is_jump {
            self.state = State::Fall;
        }

        if input.is_action_just_pressed("shoot") && self.base().is_on_floor() && direction != 0.0 {
            let mut marker = self.base().get_node_as::<Marker2D>("Muzzle");
            let curr_pos = marker.get_position();

            if direction > 0.0 && curr_pos.x.is_sign_negative()
                || direction < 0.0 && !curr_pos.x.is_sign_negative()
            {
                marker.set_position(Vector2::new(-curr_pos.x, curr_pos.y));
            }

            let mut bullet_instance = self.bullet_scene.instantiate_as::<bullet::Bullet>();
            bullet_instance.set_position(marker.get_global_position());
            self.base()
                .get_parent()
                .unwrap()
                .add_child(&bullet_instance);
            bullet_instance.bind_mut().direction = direction;
            self.state = State::Shoot;
        }

        match self.state {
            State::Idle => animation.set_animation("idle"),
            State::Shoot => animation.set_animation("run_shoot"),
            State::Run => {
                animation.set_animation("run");
            }
            State::Jump => animation.set_animation("jump"),
            State::Fall => animation.set_animation("fall"),
        }

        animation.play();
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}

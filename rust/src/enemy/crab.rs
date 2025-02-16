use godot::{
    classes::{AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D, Marker2D, ProjectSettings, Timer},
    obj::{NewGd, WithBaseField},
    prelude::*,
};

use crate::{bullet::Bullet, enemy::enemy_death::EnemyDeath};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Crab {
    state: State,
    can_walk: bool,

    direction: Vector2,
    point_positions: Vec<Vector2>,
    current_point_index: usize,
    current_point: Vector2,

    #[export]
    health_amount: i32,
    #[export]
    damage_amount: i32,
    #[export]
    speed: f32,
    #[export]
    wait_time: f64,

    enemy_death_scene: Gd<PackedScene>,

    base: Base<CharacterBody2D>,
}

enum State {
    Idle,
    Walk,
}

#[godot_api]
impl Crab {
    #[func]
    fn on_timer_timeout(&mut self) {
        self.can_walk = true;
    }

    #[func]
    fn on_area_entered(&mut self, area: Gd<Area2D>) {
        let node = area.get_parent().unwrap();
        if node.has_method("get_damage_amount") {
            let bullet = node.cast::<Bullet>();
            self.health_amount -= bullet.bind().get_damage_amount();
            godot_print!("HP {}", self.get_health_amount());
            if self.health_amount <= 0 {
                let mut enemy_death_instance = self.enemy_death_scene.instantiate_as::<EnemyDeath>();
                enemy_death_instance.set_global_position(self.base().get_global_position());
                self.base().get_parent().unwrap().add_child(&enemy_death_instance);
                self.base_mut().queue_free();
            }
        }
        godot_print!("Entered hurtbox");
    }
}

#[godot_api]
impl ICharacterBody2D for Crab {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            health_amount: 3,
            damage_amount: 1,
            state: State::Idle,
            can_walk: false,
            direction: Vector2::LEFT,
            point_positions: vec![],
            current_point_index: 0,
            current_point: Vector2::default(),
            speed: 1500.0,
            wait_time: 3.0,
            enemy_death_scene: PackedScene::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        self.enemy_death_scene = load("res://enemies/enemy_death.tscn");

        let points = self
            .base()
            .get_node_as::<Node2D>("PatrolPoints")
            .get_children();

        if points.len() == 0 {
            godot_warn!("No patrol points");
        } else {
            let points = points
                .iter_shared()
                .map(|n| {
                    let marker = n.cast::<Marker2D>();
                    return marker.get_global_position();
                })
                .collect();

            self.point_positions = points;
            self.current_point = self.point_positions[self.current_point_index];
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let gravity: f32 = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to();

        let mut animation = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let mut timer = self.base().get_node_as::<Timer>("Timer");

        let mut velocity = self.base().get_velocity();
        velocity.y += gravity * delta as f32;

        if !self.can_walk {
            velocity = velocity.move_toward(Vector2::new(0.0, velocity.y), self.speed);
            self.state = State::Idle;
        }

        if self.can_walk {
            if (self.base().get_position().x - self.current_point.x).abs() > 5.0 {
                velocity.x = self.direction.x * self.speed * delta as f32;

                self.state = State::Walk;
            } else {
                self.current_point_index += 1;
                if self.current_point_index == self.point_positions.len() {
                    self.current_point_index = 0;
                }
                self.current_point = self.point_positions[self.current_point_index];


                if self.current_point.x > self.base().get_position().x {
                    self.direction = Vector2::RIGHT;
                    animation.set_flip_h(true);
                } else {
                    self.direction = Vector2::LEFT;
                    animation.set_flip_h(false);
                }

                self.can_walk = false;
                timer.set_wait_time(self.wait_time);
                timer.start();
            }
        }

        match self.state {
            State::Idle => animation.set_animation("idle"),
            State::Walk => animation.set_animation("walk"),
        }
        
        animation.play();
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}

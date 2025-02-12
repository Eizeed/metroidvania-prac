use godot::{classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, ProjectSettings}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    state: State,
    is_jump: bool,

    base: Base<CharacterBody2D>
}

#[derive(PartialEq)]
enum State {
    Idle,
    Run,
    Jump,
    Fall,
}

const SPEED: f32 = 300.0;
const JUMP_VELOCITY: f32 = -300.0;
const JUMP_HORIZONTAL: f32 = 500.0;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            state: State::Idle,
            is_jump: false,

            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        let mut animation = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let input = Input::singleton();
        
        let gravity: f32 = ProjectSettings::singleton().get_setting("physics/2d/default_gravity").to();

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

            velocity.x = direction * SPEED;

            if self.base().is_on_floor() {
                self.state = State::Run;
                self.is_jump = false;
            }
        } else {
            velocity = velocity.move_toward(Vector2::new(0.0, velocity.y), SPEED as f32);

            if self.base().is_on_floor() {
                self.state = State::Idle;
                self.is_jump = false;
            }
        }

        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            velocity.y = JUMP_VELOCITY;
            self.state = State::Jump;
            self.is_jump = true; 
        }

        if !self.base().is_on_floor() && self.state == State::Jump {
            velocity.x += direction * JUMP_HORIZONTAL * delta as f32;
        }

        if velocity.y > 0.0 && !self.is_jump {
            self.state = State::Fall;
        }

        match self.state {
            State::Idle => animation.set_animation("idle"),
            State::Run => animation.set_animation("run"),
            State::Jump => animation.set_animation("jump"),
            State::Fall => animation.set_animation("fall"),
        }
        animation.play();
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}

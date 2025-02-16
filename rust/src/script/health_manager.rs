use godot::{obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HealthManager {
    max_health: i32,
    current_health: i32,
    base: Base<Node>,
}

#[godot_api]
impl HealthManager {
    #[signal]
    fn on_health_change(current_health: i32);

    #[func]
    pub fn decrease_health(&mut self, health_amount: i32) {
        godot_print!("CUR HEL {}", self.current_health);
        self.current_health -= health_amount;
        if self.current_health < 0 {
            self.current_health = 0;
        }

        let current_health = self.current_health;
        godot_print!("Decrease health: {}", current_health);
        self.base_mut()
            .emit_signal("on_health_change", &[current_health.to_variant()]);
    }

    #[func]
    pub fn increase_health(&mut self, health_amount: i32) {
        self.current_health += health_amount;
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }

        let current_health = self.current_health;
        godot_print!("Increase health");
        self.base_mut()
            .emit_signal("on_health_change", &[current_health.to_variant()]);
    }

    pub fn get_current_health(&self) -> i32 {
        self.current_health
    }
}

#[godot_api]
impl INode for HealthManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            max_health: 3,
            current_health: 0,
            base,
        }
    }

    fn ready(&mut self) {
        self.current_health = self.max_health
    }
}

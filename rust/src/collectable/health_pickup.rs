use godot::{obj::WithBaseField, prelude::*};

use crate::script::health_manager::HealthManager;

#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct HealthPickup {

    #[export]
    #[init(val = 1)]
    health_amount: i32,

    base: Base<Node2D>
}

#[godot_api]
impl HealthPickup {
    #[func]
    fn on_pickup_box_entered(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("Player") {
            let mut health_manager = self
                .base()
                .get_tree()
                .unwrap()
                .get_root()
                .unwrap()
                .get_node_as::<HealthManager>("GlobalHealthManager");

            health_manager.bind_mut().increase_health(self.health_amount);

            self.base_mut().queue_free();
        }
    }
}

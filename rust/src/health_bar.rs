use godot::{
    classes::{ResourceLoader, Sprite2D, Texture2D},
    obj::NewGd,
    prelude::*,
};

use crate::health_manager::HealthManager;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct HealthBar {
    heart1: Gd<Texture2D>,
    heart0: Gd<Texture2D>,
    base: Base<Node2D>,
}

#[godot_api]
impl HealthBar {
    #[func]
    fn on_player_health_change(&mut self, current_health: Variant) {
        let heart_1 = self.base().get_node_as::<Sprite2D>("Heart1");
        let heart_2 = self.base().get_node_as::<Sprite2D>("Heart2");
        let heart_3 = self.base().get_node_as::<Sprite2D>("Heart3");

        let mut hearts = [heart_1, heart_2, heart_3];

        let current_health: i32 = current_health.to();

        hearts.iter_mut().enumerate().for_each(|(idx, heart)| {
            let idx = (idx + 1) as i32;
            if idx > current_health {
                heart.set_texture(&self.heart0);
            } else {
                heart.set_texture(&self.heart1);
            }
        });
    }
}

#[godot_api]
impl INode2D for HealthBar {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            heart1: Texture2D::new_gd(),
            heart0: Texture2D::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        let mut health_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<HealthManager>("GlobalHealthManager");

        health_manager.connect(
            "on_health_change",
            &self.base().callable("on_player_health_change"),
        );

        let mut resource_loader = ResourceLoader::singleton();
        self.heart0 = resource_loader
            .load("res://ui/healthbar/tile_0046.png")
            .unwrap()
            .cast::<Texture2D>();
        self.heart1 = resource_loader
            .load("res://ui/healthbar/tile_0044.png")
            .unwrap()
            .cast::<Texture2D>();
    }
}

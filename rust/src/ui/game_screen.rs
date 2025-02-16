use godot::{
    classes::{CanvasLayer, ICanvasLayer, Label},
    prelude::*,
};

use crate::script::{collectable_manager::CollectableManager, game_manager::GameManager};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
struct GameScreen {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl GameScreen {
    #[func]
    fn on_score_changed(&mut self) {
        let collectable_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<CollectableManager>("GlobalCollectableManager");

        let root = self.base().get_tree().unwrap().get_root().unwrap();

        let level = root
            .get_children()
            .iter_shared()
            .find(|node| node.is_class("Level"))
            .unwrap();
        level
            .get_node_as::<Label>("GameScreen/MarginContainer/VBoxContainer/HBoxContainer/Label")
            .set_text(&collectable_manager.bind().get_total_score().to_string());
    }

    #[func]
    fn on_pause_button_pressed(&mut self) {
        let mut game_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<GameManager>("GlobalGameManager");

        game_manager.bind_mut().pause_game();
    }
}

#[godot_api]
impl ICanvasLayer for GameScreen {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let mut collectable_manager =
            self.base()
                .get_tree()
                .unwrap()
                .get_root()
                .unwrap()
                .get_node_as::<CollectableManager>("GlobalCollectableManager");

        collectable_manager.connect("score_change", &self.base().callable("on_score_changed"));
    }
}

use godot::{classes::{node::ProcessMode, CanvasLayer, ICanvasLayer}, obj::WithBaseField, prelude::*};

use crate::script::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=CanvasLayer, init)]
pub struct MainMenu {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl MainMenu {
    #[func]
    fn on_play_pressed(&mut self) {
        let mut game_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<GameManager>("GlobalGameManager");

        game_manager.bind_mut().start_game();

        self.base_mut().queue_free();
    }

    #[func]
    fn on_exit_pressed(&mut self) {
        let mut game_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<GameManager>("GlobalGameManager");

        game_manager.bind_mut().exit_game();
    }
}

#[godot_api]
impl ICanvasLayer for MainMenu {
    fn ready(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::ALWAYS);
    }
}

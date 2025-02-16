use godot::{
    classes::{node::ProcessMode, CanvasLayer, ICanvasLayer}, obj::WithBaseField, prelude::*
};

use crate::script::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=CanvasLayer, init)]
pub struct PauseMenu {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl PauseMenu {
    #[func]
    fn on_continue_pressed(&mut self) {
        let mut game_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<GameManager>("GlobalGameManager");

        game_manager.bind_mut().continue_game();

        self.base_mut().queue_free();
    }

    #[func]
    fn on_main_menu_pressed(&mut self) {
        let mut game_manager = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<GameManager>("GlobalGameManager");

        game_manager.bind_mut().main_menu();

        self.base_mut().queue_free();
    }
}

#[godot_api]
impl ICanvasLayer for PauseMenu {
    fn ready(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::ALWAYS);
    }
}

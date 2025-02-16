use godot::{
    classes::RenderingServer,
    obj::{NewGd, WithBaseField},
    prelude::*,
};

use crate::{
    level::Level,
    ui::{main_menu::MainMenu, pause_menu::PauseMenu},
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    pause_menu: Gd<PackedScene>,
    main_menu: Gd<PackedScene>,
    base: Base<Node>,
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn start_game(&mut self) {
        let level_1: Gd<PackedScene> = load("res://levels/level_1.tscn");
        if self.base().get_tree().unwrap().is_paused() {
            self.base().get_tree().unwrap().set_pause(false);
            return;
        }

        self.transition_to_scene(level_1);
    }

    #[func]
    pub fn exit_game(&mut self) {
        self.base().get_tree().unwrap().quit();
    }

    #[func]
    pub fn transition_to_scene(&mut self, scene: Gd<PackedScene>) {
        let instance = scene.instantiate_as::<Level>();
        self.base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .add_child(&instance);
    }

    #[func]
    pub fn pause_game(&mut self) {
        let mut tree = self.base().get_tree().unwrap();
        let instance = self.pause_menu.instantiate_as::<PauseMenu>();
        tree.get_root().unwrap().add_child(&instance);
        tree.set_pause(true);
    }

    #[func]
    pub fn continue_game(&mut self) {
        let mut tree = self.base().get_tree().unwrap();
        tree.set_pause(false);
    }

    #[func]
    pub fn main_menu(&mut self) {
        let mut tree = self.base().get_tree().unwrap();
        let instance = self.main_menu.instantiate_as::<MainMenu>();
        tree.get_root().unwrap().add_child(&instance);
        tree.set_pause(true);
    }
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            pause_menu: PackedScene::new_gd(),
            main_menu: PackedScene::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        self.pause_menu = load("res://ui/pause_menu.tscn");
        self.main_menu = load("res://ui/main_menu.tscn");
        RenderingServer::singleton().set_default_clear_color(Color::from_rgb(0.44, 0.12, 0.53));
    }
}

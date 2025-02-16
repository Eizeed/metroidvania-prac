use godot::{
    classes::{AnimatedSprite2D, Label},
    prelude::*,
};

use crate::script::collectable_manager::CollectableManager;

#[derive(GodotClass)]
#[class(base=Node2D, init)]
struct BlueCoin {
    #[export]
    #[init(val = 1)]
    score_amount: i32,

    base: Base<Node2D>,
}

#[godot_api]
impl BlueCoin {
    #[func]
    fn on_pickup_box_entered(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("Player") {
            self.base()
                .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
                .hide();

            let mut label = self.base().get_node_as::<Label>("Label");
            label.set_text(&self.score_amount.to_string());
            label.show();

            let mut collectable_manager = self
                .base()
                .get_tree()
                .unwrap()
                .get_root()
                .unwrap()
                .get_node_as::<CollectableManager>("GlobalCollectableManager");

            collectable_manager.bind_mut().give_pickup_award(self.score_amount);

            let mut tween = self.base().get_tree().unwrap().create_tween().unwrap();
            let mut label_pos = label.get_position();
            label_pos.y -= 10.0;
            tween
                .tween_property(&label, "position", &label_pos.to_variant(), 0.5)
                .unwrap()
                .from_current();

            tween.tween_callback(&self.base().callable("queue_free"));
        }
    }
}

#[godot_api]
impl INode2D for BlueCoin {
    fn ready(&mut self) {
        self.base().get_node_as::<Label>("Label").hide();
    }
}

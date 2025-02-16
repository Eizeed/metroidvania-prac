use godot::{obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct CollectableManager {
    total_score: i32,
    base: Base<Node>,
}

#[godot_api]
impl INode for CollectableManager {
    fn ready(&mut self) {
        let on_score_changed = &self.base().callable("on_score_changed");
        self.base_mut().connect("score_change", on_score_changed);
        
        godot_print!("omfg");
    }
}

#[godot_api]
impl CollectableManager {
    #[signal]
    fn score_change();

    #[func]
    pub fn give_pickup_award(&mut self, score: i32) {
        self.total_score += score;
        self.base_mut().emit_signal("score_change", &[]);
    }

    pub fn get_total_score(&self) -> i32 {
        return self.total_score
    }
}

use super::*;

use godot::classes::Label;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct Singleton;

#[godot_api]
impl Singleton {
    #[func]
    pub fn example_concrete(&self, node: Gd<MyNode>, mut label: Gd<Label>) {
        label.set_text(&format!("test: {:.3}", node.bind().get_example_export()));
    }
}

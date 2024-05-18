mod classes;
mod types;

use godot::prelude::*;

struct Game;

#[gdextension]
unsafe impl ExtensionLibrary for Game {}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Main {
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

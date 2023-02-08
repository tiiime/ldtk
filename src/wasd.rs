use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct WasdPlugin;

impl Plugin for WasdPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_plugin(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
    Jump,
}

#[derive(Bundle)]
pub struct InputBundle {
    #[bundle]
    input_manager: InputManagerBundle<Action>,
}

impl Default for InputBundle {
    fn default() -> Self {
        use Action::*;
        Self {
            input_manager: InputManagerBundle {
                input_map: InputMap::new([
                    (KeyCode::W, Up),
                    (KeyCode::A, Left),
                    (KeyCode::S, Down),
                    (KeyCode::D, Right),
                    (KeyCode::Space, Jump),
                ]),
                ..default()
            },
        }
    }
}
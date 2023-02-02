use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
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
                ]),
                ..default()
            },
        }
    }
}

pub fn handle_common_wasd_transform(action: &ActionState<Action>, mut transform: Mut<Transform>) {
    let mut x = 0;
    let mut y = 0;
    if action.pressed(Action::Up) {
        y += 1
    }
    if action.pressed(Action::Left) {
        x -= 1
    }
    if action.pressed(Action::Down) {
        y -= 1
    }
    if action.pressed(Action::Right) {
        x += 1
    };

    transform.translation.x += x as f32;
    transform.translation.y += y as f32;
}

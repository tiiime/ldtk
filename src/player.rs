use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{components, wasd};

use components::*;
use wasd::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_system(leafwing_input);
    }
}

const MOVE_SPEED: f32 = 200.;

fn leafwing_input(mut player_query: Query<(&mut Velocity, &ActionState<Action>), With<Player>>) {
    if player_query.is_empty() {
        return;
    }

    let (mut velocity, action) = player_query.single_mut();

    if action.pressed(wasd::Action::Left) {
        velocity.linvel.x = -MOVE_SPEED;
    }
    if action.pressed(wasd::Action::Right) {
        velocity.linvel.x = MOVE_SPEED;
    }
    if action.pressed(wasd::Action::Jump) {
        velocity.linvel.y = MOVE_SPEED
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    sprite: SpriteSheetBundle,
    player: Player,

    #[bundle]
    input: wasd::InputBundle,

    #[bundle]
    rapier: PlayerRapierBundle,
}

#[derive(Bundle)]
struct PlayerRapierBundle {
    // rigid_body:RigidBody,
    velocity: Velocity,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
}

impl Default for PlayerRapierBundle {
    fn default() -> Self {
        Self {
            velocity: Velocity::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(9., 16.),
            locked_axes: LockedAxes::ROTATION_LOCKED_Z,
        }
    }
}

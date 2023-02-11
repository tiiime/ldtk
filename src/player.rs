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
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_system(leafwing_input)
            .add_system(ground_detection);
    }
}

const MOVE_SPEED: f32 = 80.;

fn leafwing_input(
    mut player_query: Query<
        (
            &mut Velocity,
            &ActionState<Action>,
            &mut JumpFlag,
            &Grounded,
        ),
        With<Player>,
    >,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut velocity, action, mut jump_flag, grounded) = player_query.single_mut();

    let speed = if action.pressed(wasd::Action::Speed) {
        MOVE_SPEED * 2.
    } else {
        MOVE_SPEED
    };

    if action.pressed(wasd::Action::Left) {
        velocity.linvel.x = -speed;
    }
    if action.pressed(wasd::Action::Right) {
        velocity.linvel.x = speed;
    }
    if action.just_pressed(wasd::Action::Jump) && jump_flag.can_jump() {
        velocity.linvel.y = 160.;
        jump_flag.increase_jump_count();
    }
    if grounded.0 {
        jump_flag.clear()
    }
}

/// 太 6 了，地面检测是 y 轴几帧不动
fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<(f32, isize)>,
) {
    if player.is_empty() {
        return;
    }

    let (pos, mut on_ground) = player.single_mut();

    if (pos.translation.y * 100.).round() == last.0 {
        last.1 += 1;
    } else {
        last.1 -= 1;
    };
    last.1 = last.1.clamp(0, 5);

    if last.1 == 5 && !on_ground.0 {
        on_ground.0 = true;
    } else if last.1 < 2 && on_ground.0 {
        on_ground.0 = false;
    }

    last.0 = (pos.translation.y * 100.).round();
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    sprite: SpriteSheetBundle,
    player: Player,

    jump_flag: JumpFlag,
    grounded: Grounded,

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

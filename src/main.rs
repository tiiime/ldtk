use bevy::{ecs::bundle, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod wasd;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(InputManagerPlugin::<wasd::Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Uid(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_system(leafwing_input)
        .run();
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());

    command.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("samples.ldtk"),
        ..default()
    });
}

const MOVE_SPEED: f32 = 200.;

fn leafwing_input(
    mut player_query: Query<(&mut Velocity, &ActionState<wasd::Action>), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut velocity, action) = player_query.single_mut();

    if action.pressed(wasd::Action::Left) {
        velocity.linvel.x = -MOVE_SPEED;
    } else if action.pressed(wasd::Action::Right) {
        velocity.linvel.x = MOVE_SPEED;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Component)]
pub struct Player;

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    #[sprite_bundle("player.png")]
    #[bundle]
    sprite_bundle: SpriteBundle,

    player: Player,

    #[bundle]
    input: wasd::InputBundle,

    #[bundle]
    rapier: RapierBundle,
}

#[derive(Bundle)]
struct RapierBundle {
    // rigid_body:RigidBody,
    velocity: Velocity,
    rigid_body: RigidBody,
}

impl Default for RapierBundle {
    fn default() -> Self {
        Self {
            velocity: Velocity::default(),
            rigid_body: RigidBody::Dynamic,
        }
    }
}

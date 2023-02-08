use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod animation;
mod collision;
mod wasd;

use collision::*;
use animation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_editor_pls::prelude::EditorPlugin)
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InputManagerPlugin::<wasd::Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(AnimationPlugin)
        .add_plugin(FixedBlockCollisionPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::Y * -294.,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 10,
            },
            ..Default::default()
        })
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Uid(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<CellBundle>(1)
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
    } 
    if action.pressed(wasd::Action::Right) {
        velocity.linvel.x = MOVE_SPEED;
    } 
    if action.pressed(wasd::Action::Jump) {
        velocity.linvel.y = MOVE_SPEED
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Component)]
pub struct Player;

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

#[derive(Debug, LdtkIntCell, Bundle)]
struct CellBundle {
    block: FixedBlock,
}

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;

mod wasd;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(InputManagerPlugin::<wasd::Action>::default())
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

fn leafwing_input(
    mut player_query: Query<(&mut Transform, &ActionState<wasd::Action>), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (transform, action) = player_query.single_mut();

    wasd::handle_common_wasd_transform(action, transform)
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
}

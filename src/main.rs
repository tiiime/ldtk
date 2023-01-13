use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Uid(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_system(player_move)
        .run();
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());

    command.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("samples.ldtk"),
        ..default()
    });
}

fn player_move(input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform), With<Player>>) {
    let mut x = 0;
    let mut y = 0;
    if input.pressed(KeyCode::W) {
        y += 1
    }
    if input.pressed(KeyCode::A) {
        x -= 1
    }
    if input.pressed(KeyCode::S) {
        y -= 1
    }
    if input.pressed(KeyCode::D) {
        x += 1
    };
    for mut item in query.iter_mut() {
        item.translation.x += x as f32;
        item.translation.y += y as f32;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_bundle("player.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
}

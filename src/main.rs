use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;

mod animation;
mod collision;
mod components;
mod player;
mod wasd;

use animation::*;
use collision::*;
use player::*;
use wasd::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        // 开发 inspect
        .add_plugin(bevy_editor_pls::prelude::EditorPlugin)
        .add_plugin(InspectableRapierPlugin)
        // 游戏逻辑插件
        .add_plugin(AnimationPlugin)
        .add_plugin(FixedBlockCollisionPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WasdPlugin)
        // 物理引擎插件
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::Y * -294.,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 10,
            },
            ..Default::default()
        })
        // Ldtk 配置
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Uid(0))
        .run();
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());

    command.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("samples.ldtk"),
        ..default()
    });
}

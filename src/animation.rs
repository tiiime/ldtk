use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Player;

///
/// ref: https://github.com/PhaestusFox/bevy_platformer
///
/// 动画插件
/// [animate_sprite]: 更新动画帧
/// []
///
#[derive(Debug)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationResource>()
            .add_system(animate_sprite)
            .add_system(append_animation_for_player)
            .add_system(change_player_animation);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum AnimationState {
    Idle,
    Jump,
    Run,
    Fall,
}

#[derive(Debug, Clone, Component)]
struct AnimationMeta {
    len: usize,
    frame_time: f32,
}

impl AnimationMeta {
    fn new(len: usize, fps: usize) -> AnimationMeta {
        AnimationMeta {
            len: len,
            frame_time: 1. / (fps as f32),
        }
    }
}

///
/// 加载动画资源 Resource
///
#[derive(Debug, Resource)]
struct AnimationResource {
    map: HashMap<AnimationState, (Handle<TextureAtlas>, AnimationMeta)>,
}

impl AnimationResource {
    fn add(&mut self, state: AnimationState, handle: Handle<TextureAtlas>, meta: AnimationMeta) {
        self.map.insert(state, (handle, meta));
    }
    fn get(&self, state: AnimationState) -> Option<(Handle<TextureAtlas>, AnimationMeta)> {
        self.map.get(&state).cloned()
    }
}

impl FromWorld for AnimationResource {
    fn from_world(world: &mut World) -> Self {
        let mut res = AnimationResource {
            map: HashMap::new(),
        };
        world.resource_scope(|world, mut texture_atles: Mut<Assets<TextureAtlas>>| {
            let asset_server = world.resource::<AssetServer>();

            // Virtual Guy
            let idel_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Virtual Guy/Idle (32x32).png"),
                Vec2::splat(32.),
                11,
                1,
                None,
                None,
            );
            res.add(
                AnimationState::Idle,
                texture_atles.add(idel_atlas),
                AnimationMeta::new(11, 20),
            );

            let run_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Virtual Guy/Run (32x32).png"),
                Vec2::splat(32.),
                12,
                1,
                None,
                None,
            );
            res.add(
                AnimationState::Run,
                texture_atles.add(run_atlas),
                AnimationMeta::new(12, 20),
            );

            let jump_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Virtual Guy/Jump (32x32).png"),
                Vec2::splat(32.),
                1,
                1,
                None,
                None,
            );
            res.add(
                AnimationState::Jump,
                texture_atles.add(jump_atlas),
                AnimationMeta::new(1, 1),
            );

            // let djump_atlas = TextureAtlas::from_grid(
            //     asset_server.load("Main Characters/Virtual Guy/Double Jump (32x32).png"),
            //     Vec2::splat(32.),
            //     6,
            //     1,
            //     None,
            //     None,
            // );
            // res.add(
            //     Animation::GuyDubbleJump,
            //     texture_atles.add(djump_atlas),
            //     AnimationMeta{len:6,frame_time: 20.},
            // );

            let fall_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Virtual Guy/Fall (32x32).png"),
                Vec2::splat(32.),
                1,
                1,
                None,
                None,
            );
            res.add(
                AnimationState::Fall,
                texture_atles.add(fall_atlas),
                AnimationMeta::new(1, 1),
            )
        });
        res
    }
}

#[derive(Component)]
struct FrameTime(pub f32);

///
/// 添加 [PhoxAnimationBundle] 到指定 entity，就可以播放动画
/// 包含动画相关元信息 [AnimationMeta]
/// 以及当前引擎执行过帧的记数 [FrameTime]，初始为 0
///
#[derive(Bundle)]
struct PhoxAnimationBundle {
    animaiton: AnimationMeta,
    frame_time: FrameTime,
}

impl PhoxAnimationBundle {
    fn new(animaiton: AnimationMeta) -> PhoxAnimationBundle {
        PhoxAnimationBundle {
            animaiton,
            frame_time: FrameTime(0.0),
        }
    }
}

///
/// 更新 [PhoxAnimationBundle] 内 [FrameTime]
/// 根据 frame_time 计算动画 index
/// 更新 [TextureAtlasSprite] index
///
fn animate_sprite(
    mut animations: Query<(&mut TextureAtlasSprite, &AnimationMeta, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in animations.iter_mut() {
        let delt = time.delta_seconds();
        frame_time.0 += delt;
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }
            frame_time.0 -= animation.frame_time;
        }
    }
}

///
/// 为没有 [AnimationMeta] 的 [Player] entity 添加动画信息
///
fn append_animation_for_player(
    mut commands: Commands,
    mut query: Query<Entity, (With<Player>, Without<AnimationMeta>)>,
    animations: Res<AnimationResource>,
) {
    if query.is_empty() {
        return;
    }
    let entity = query.single_mut();

    let Some((_texture_atlas, animation)) = animations.get(AnimationState::Idle) else {    error!("Failed to find animation: Idle");        return;};

    commands
        .entity(entity)
        .insert(PhoxAnimationBundle::new(animation));
}

///
/// 更新动画状态
///
fn change_player_animation(
    mut player: Query<
        (
            &Player,
            &mut Handle<TextureAtlas>,
            &mut AnimationMeta,
            &mut TextureAtlasSprite,
            &Velocity,
        ),
        (With<Player>, With<AnimationMeta>),
    >,
    animaitons: Res<AnimationResource>,
) {
    if player.is_empty() {
        return;
    }
    let (_player, mut atlas, mut animation, mut sprite, velocity) = player.single_mut();
    if velocity.linvel.x < -0.1 {
        sprite.flip_x = true;
    } else if velocity.linvel.x > 0.1 {
        sprite.flip_x = false;
    }

    let set = if velocity.linvel.y > 0.01 {
        //Jumping if jump
        AnimationState::Jump
    } else if velocity.linvel.y < -0.01 {
        //Falling if no on ground
        AnimationState::Fall
    } else if velocity.linvel.x != 0.0 {
        // Animation::MaskFall
        // if any move keys pressed set run sprite
        AnimationState::Run
    } else {
        AnimationState::Idle
    };

    let Some((new_atlas, new_animaiton)) = animaitons.get(set) else {error!("No Animation Jump Loaded"); return;};
    *atlas = new_atlas;
    sprite.index %= new_animaiton.len;
    *animation = new_animaiton;
}

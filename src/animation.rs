use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::Player;

#[derive(Debug, Hash, PartialEq, Eq)]
enum AnimationState {
    Idle,
    Jump,
    Run,
}

#[derive(Debug, Clone, Component)]
pub struct AnimationMeta {
    len: usize,
    frame_time: f32,
}

#[derive(Debug, Resource)]
pub struct AnimationResource {
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
                AnimationMeta {
                    len: 11,
                    frame_time: 1. / 20.,
                },
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
                AnimationMeta {
                    len: 12,
                    frame_time: 1. / 20.,
                },
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
                AnimationMeta {
                    len: 1,
                    frame_time: 1.,
                },
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

            // let fall_atlas = TextureAtlas::from_grid(
            //     asset_server.load("Main Characters/Virtual Guy/Fall (32x32).png"),
            //     Vec2::splat(32.),
            //     1,
            //     1,
            //     None,
            //     None,
            // );
        });
        res
    }
}

#[derive(Component)]
pub struct FrameTime(pub f32);

#[derive(Bundle)]
pub struct PhoxAnimationBundle {
    pub animaiton: AnimationMeta,
    frame_time: FrameTime,
}

impl PhoxAnimationBundle {
    pub fn new(animaiton: AnimationMeta) -> PhoxAnimationBundle {
        PhoxAnimationBundle {
            animaiton,
            frame_time: FrameTime(0.0),
        }
    }
}

pub fn animate_sprite(
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

pub fn append_animation_for_player(
    mut commands: Commands,
    mut query: Query<(Entity), (With<Player>, Without<AnimationMeta>)>,
    animations: Res<AnimationResource>,
) {
    if query.is_empty() {
        return;
    }
    let (entity) = query.single_mut();

    let Some((texture_atlas, animation)) = animations.get(AnimationState::Idle) else {    error!("Failed to find animation: Idle");        return;};

    commands
        .entity(entity)
        .insert(PhoxAnimationBundle::new(animation));
}

pub fn change_player_animation(
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
    let (player, mut atlas, mut animation, mut sprite, velocity) = player.single_mut();
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
        AnimationState::Idle
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

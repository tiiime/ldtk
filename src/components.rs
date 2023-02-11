use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Component)]
pub struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
pub struct Grounded(pub bool);

impl Default for Grounded {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
pub struct JumpFlag {
    // 跳跃次数
    pub jump_count: usize,
    // 跳跃次数限制
    jump_limit: usize,
}

impl Default for JumpFlag {
    fn default() -> Self {
        Self {
            jump_count: 0,
            // 三段跳，兄弟！
            jump_limit: 2,
        }
    }
}

impl JumpFlag {
    pub(crate) fn can_jump(&self) -> bool {
        self.jump_count < self.jump_limit
    }

    pub(crate) fn increase_jump_count(&mut self) {
        self.jump_count += 1;
    }

    pub(crate) fn clear(&mut self) {
        self.jump_count = 0;
    }
}

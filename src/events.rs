use bevy::prelude::*;

use crate::particles::ParticleEffect;

pub struct SwitchMagic;

pub struct SwitchWeapon;

pub struct PlayerCollision {
    pub player: Entity,
    pub other: Entity,
}

pub struct MagicCollision {
    pub magic: Entity,
    pub other: Entity,
}

impl MagicCollision {
    pub fn new(magic: Entity, other: Entity) -> Self {
        Self { magic, other }
    }
}

pub struct WeaponCollision {
    pub weapon: Entity,
    pub other: Entity,
}

impl WeaponCollision {
    pub fn new(weapon: Entity, other: Entity) -> Self {
        Self { weapon, other }
    }
}

pub struct EmitParticleEffect {
    pub ty: ParticleEffect,
    pub pos: Vec3,
    pub offset: Vec3,
}

pub struct KillAttackable(pub Entity);

pub struct DamageAttackable(pub Entity);

pub struct DamagePlayer(pub Entity);

impl EmitParticleEffect {
    pub fn new(ty: ParticleEffect, pos: Vec3) -> Self {
        Self {
            ty,
            pos,
            offset: Vec3::ZERO,
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}

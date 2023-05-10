use bevy::prelude::*;

use crate::particles::ParticleEffect;

pub struct SwitchMagic;

pub struct SwitchWeapon;

pub struct PlayerCollision {
    pub player: Entity,
    pub other: Entity,
}

impl PlayerCollision {
    pub fn new(player: Entity, other: Entity) -> Self {
        Self { player, other }
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
}

impl EmitParticleEffect {
    pub fn new(ty: ParticleEffect, pos: Vec3) -> Self {
        Self { ty, pos }
    }
}

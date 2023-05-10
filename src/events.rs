use bevy::prelude::Entity;

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

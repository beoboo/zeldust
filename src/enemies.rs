pub enum AttackType {
    Slash,
    Claw,
    Thunder,
    Leaf,
}

impl AttackType {
    pub fn sound(&self) -> &str {
        match self {
            AttackType::Slash => "slash",
            AttackType::Claw => "claw",
            AttackType::Thunder => "fireball",
            AttackType::Leaf => "slash",
        }
    }
}

pub enum EnemyType {
    Squid,
    Raccoon,
    Spirit,
    Bamboo,
}

impl EnemyType {
    pub fn health(&self) -> u32 {
        match self {
            EnemyType::Squid => 100,
            EnemyType::Raccoon => 300,
            EnemyType::Spirit => 100,
            EnemyType::Bamboo => 70,
        }
    }

    pub fn exp(&self) -> u32 {
        match self {
            EnemyType::Squid => 100,
            EnemyType::Raccoon => 250,
            EnemyType::Spirit => 110,
            EnemyType::Bamboo => 120,
        }
    }

    pub fn damage(&self) -> u32 {
        match self {
            EnemyType::Squid => 20,
            EnemyType::Raccoon => 40,
            EnemyType::Spirit => 8,
            EnemyType::Bamboo => 6,
        }
    }

    pub fn speed(&self) -> u32 {
        match self {
            EnemyType::Squid => 3,
            EnemyType::Raccoon => 2,
            EnemyType::Spirit => 4,
            EnemyType::Bamboo => 3,
        }
    }

    pub fn resistance(&self) -> u32 {
        match self {
            EnemyType::Squid => 3,
            EnemyType::Raccoon => 3,
            EnemyType::Spirit => 3,
            EnemyType::Bamboo => 3,
        }
    }

    pub fn attack_radius(&self) -> u32 {
        match self {
            EnemyType::Squid => 80,
            EnemyType::Raccoon => 120,
            EnemyType::Spirit => 60,
            EnemyType::Bamboo => 50,
        }
    }

    pub fn notice_radius(&self) -> u32 {
        match self {
            EnemyType::Squid => 360,
            EnemyType::Raccoon => 400,
            EnemyType::Spirit => 350,
            EnemyType::Bamboo => 300,
        }
    }
}

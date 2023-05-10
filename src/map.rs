use bevy::prelude::*;
use bevy_rapier2d::parry::utils::hashmap::HashMap;
use rand::Rng;

use crate::{debug::VALID_LAYERS, layer::Layer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum LayerType {
    Blocks,
    Entities,
    Grass,
    Objects,
}

impl LayerType {
    pub fn to_index(&self, index: usize) -> usize {
        match self {
            LayerType::Grass => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..3)
            },
            _ => index,
        }
    }

    pub fn is_attackable(&self) -> bool {
        matches!(*self, LayerType::Grass)
    }
}

#[derive(Resource)]
pub struct WorldMap {
    pub layers: HashMap<LayerType, Layer>,
}

impl WorldMap {
    pub fn new() -> Self {
        Self {
            layers: HashMap::default(),
        }
    }

    pub fn load_layer(self, ty: LayerType, path: &str) -> Self {
        let layer = Layer::load(path);

        let mut layers = self.layers;

        if VALID_LAYERS.contains(&ty) {
            layers.insert(ty, layer);
        }

        Self { layers }
    }

    pub fn simple() -> Self {
        #[rustfmt::skip]
            let data = vec![
            vec![20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20],
            vec![20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, 30, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, 20, 20, 20, 20, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, 20, 20, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, -1, 20, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, 20, 20, 20, 20, 20, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, -1, 20, 20, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20],
            vec![20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20],
        ];

        let layer = Layer { data };

        let mut layers = HashMap::default();
        layers.insert(LayerType::Blocks, layer);

        Self { layers }
    }

    pub fn debug_enemy() -> Self {
        let mut layers = HashMap::default();

        layers.insert(LayerType::Blocks, Self::empty_layer());
        layers.insert(LayerType::Entities, Self::one_enemy_layer());

        Self { layers }
    }

    pub fn debug_grass() -> Self {
        let mut layers = HashMap::default();

        layers.insert(LayerType::Blocks, Self::empty_layer());
        layers.insert(LayerType::Grass, Self::one_grass_layer());
        layers.insert(LayerType::Entities, Self::no_enemy_layer());

        Self { layers }
    }

    fn no_enemy_layer() -> Layer {
        #[rustfmt::skip]
            let data = vec![
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, 394, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
        ];

        Layer { data }
    }

    fn one_enemy_layer() -> Layer {
        #[rustfmt::skip]
            let data = vec![
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, 394, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, 391, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
        ];

        Layer { data }
    }

    fn one_grass_layer() -> Layer {
        #[rustfmt::skip]
            let data = vec![
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, 20, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
        ];

        Layer { data }
    }

    fn empty_layer() -> Layer {
        #[rustfmt::skip]
            let data = vec![
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1],
        ];

        Layer { data }
    }

    fn boxed_layer() -> Layer {
        #[rustfmt::skip]
            let data = vec![
            vec![395, 395, 395, 395, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, -1, -1, -1, 395],
            vec![395, 395, 395, 395, 395],
        ];

        Layer { data }
    }
}

use bevy::prelude::*;
use bevy_rapier2d::parry::utils::hashmap::HashMap;
use rand::Rng;

use crate::layer::Layer;

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
            }
            _ => index,
        }
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
        layers.insert(ty, layer);

        Self { layers }
    }
}

impl Default for WorldMap {
    fn default() -> Self {
        let data = vec![
            vec![
                20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, 30, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, 20, 20, 20, 20, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, 20, 20, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, 20, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, 20, 20, 20, 20, 20, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, 20, 20, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20,
            ],
            vec![
                20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
            ],
        ];

        let layer = Layer { data };

        let mut layers = HashMap::default();
        layers.insert(LayerType::Blocks, layer);

        Self { layers }
    }
}

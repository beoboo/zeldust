use crate::layer::Layer;

#[derive(Default)]
pub struct WorldMap {
    pub layers: Vec<Layer>
}

impl WorldMap {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
        }
    }

    pub fn load_layer(self, path: &str) -> Self {
        let layer = Layer::load(path);

        let mut layers = self.layers;
        layers.push(layer);

        Self {
            layers
        }
    }
}
//
// impl Default for WorldMap {
//     fn default() -> Self {
//         let data = vec![
//             "xxxxxxxxxxxxxxxxxxxx".to_string(),
//             "x                  x".to_string(),
//             "x p                x".to_string(),
//             "x  x     xxxxx     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         x     x".to_string(),
//             "x  x         xxx   x".to_string(),
//             "x      x x         x".to_string(),
//             "x     xxxxx        x".to_string(),
//             "x      xxx         x".to_string(),
//             "x       x          x".to_string(),
//             "x                  x".to_string(),
//             "x                  x".to_string(),
//             "x                  x".to_string(),
//             "xxxxxxxxxxxxxxxxxxxx".to_string(),
//         ];
//
//         Self {
//             layers: vec![]
//         }
//     }
// }
use std::collections::BTreeMap;

use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "1c363550-1333-4889-8145-2633d881c243"]
pub struct TexturePack {
    pub frames: BTreeMap<String, TextureFrame>,
}

impl TexturePack {
    pub fn index_of(&self, frame: &str) -> usize {
        for (id, key) in self.frames.keys().enumerate() {
            if key == frame {
                return id;
            }
        }

        unreachable!("Cannot find index of {frame}")
    }
}

#[derive(Debug, Deserialize)]
pub struct TextureFrame {
    pub frame: Frame,
}

#[derive(Debug, Deserialize)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn load_frames() -> Result<()> {
        let data = r#"
{
  "frames": {
    "0.png": {
      "frame": {
        "x": 0,
        "y": 0,
        "w": 128,
        "h": 128
      }
    }
  }
}"#;
        let tile_set = serde_json::from_str::<TexturePack>(data)?;

        assert_eq!(tile_set.frames.len(), 1);
        let tile = &tile_set.frames["0.png"];
        assert_eq!(tile.frame.x, 0);

        Ok(())
    }
}
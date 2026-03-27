use serde::{Deserialize, Serialize};
use seiscache_core::ChunkKey;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadPlan {
    pub key: ChunkKey,
    pub byte_len: usize,
    pub target: UploadTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadTarget {
    Buffer,
    Texture,
}

impl UploadPlan {
    pub fn buffer(key: ChunkKey, byte_len: usize) -> Self {
        Self {
            key,
            byte_len,
            target: UploadTarget::Buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_upload_plan_uses_buffer_target() {
        let plan = UploadPlan::buffer(
            ChunkKey {
                dataset_id: "demo".to_string(),
                iline_chunk: 0,
                xline_chunk: 0,
                sample_chunk: 1,
            },
            1024,
        );
        assert_eq!(plan.target, UploadTarget::Buffer);
    }
}


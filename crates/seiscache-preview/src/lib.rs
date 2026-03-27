use serde::{Deserialize, Serialize};
use seiscache_core::CacheStats;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PreviewKey {
    pub dataset_id: String,
    pub section_id: String,
    pub parameter_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviewRecord {
    pub key: PreviewKey,
    pub bytes_ready: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviewCacheReport {
    pub records: usize,
    pub stats: CacheStats,
}

impl PreviewCacheReport {
    pub fn has_reuse(&self) -> bool {
        self.stats.hits > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_report_detects_reuse() {
        let report = PreviewCacheReport {
            records: 1,
            stats: CacheStats { hits: 2, misses: 1 },
        };
        assert!(report.has_reuse());
    }
}


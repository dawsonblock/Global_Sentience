//! Symbolic compression — efficient encoding of traces and graphs.

use serde::{Deserialize, Serialize};

/// Statistics about a compression operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: usize,
    pub compressed_size: usize,
    pub ratio: f64,
}

impl CompressionStats {
    pub fn new(original_size: usize, compressed_size: usize) -> Self {
        let ratio = if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            1.0
        };
        Self {
            original_size,
            compressed_size,
            ratio,
        }
    }
}

/// Contains compressed symbolic representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicCompression {
    pub original_len: usize,
    pub compressed_len: usize,
    pub stats: CompressionStats,
}

impl SymbolicCompression {
    pub fn new(original_len: usize, compressed_len: usize) -> Self {
        let stats = CompressionStats::new(original_len, compressed_len);
        Self {
            original_len,
            compressed_len,
            stats,
        }
    }
}

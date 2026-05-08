//! Memory crate: semantic key-value store with keyword-scored query.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Lightweight semantic memory seeded with humanity context.
#[derive(Debug, Default)]
pub struct SemanticMemory {
    store: HashMap<String, String>,
}

impl SemanticMemory {
    pub fn new() -> Self {
        let mut m = SemanticMemory::default();
        m.seed_humanity_context();
        m
    }

    fn seed_humanity_context(&mut self) {
        self.store.entry("humanity:cooperation".into()).or_insert_with(|| {
            "People often resolve conflict through clarification, repair, mutual aid, and shared rules.".into()
        });
        self.store.entry("humanity:kindness".into()).or_insert_with(|| {
            "Kind action prioritises harm reduction, dignity, truthfulness, and patience.".into()
        });
        self.store.entry("humanity:uncertainty".into()).or_insert_with(|| {
            "Ambiguous behaviour should be handled with clarification before assigning negative intent.".into()
        });
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.store.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(key).map(String::as_str)
    }

    /// Keyword-scored query (exact word overlap, case-insensitive).
    pub fn query(&self, text: &str, limit: usize) -> Vec<SemanticHit> {
        let words: Vec<String> = text
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .collect();

        let mut scored: Vec<(usize, &str, &str)> = self
            .store
            .iter()
            .filter_map(|(k, v)| {
                let hay = format!("{} {}", k, v).to_lowercase();
                let s = words.iter().filter(|w| hay.contains(w.as_str())).count();
                if s > 0 { Some((s, k.as_str(), v.as_str())) } else { None }
            })
            .collect();

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored
            .into_iter()
            .take(limit)
            .map(|(score, key, value)| SemanticHit { key: key.into(), value: value.into(), score })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticHit {
    pub key:   String,
    pub value: String,
    pub score: usize,
}

/// Fast semantic cache keyed by a 64-bit hash of normalised text + state hint.
#[derive(Debug, Default)]
pub struct SemanticCache {
    cache: HashMap<u64, serde_json::Value>,
}

impl SemanticCache {
    pub fn new() -> Self { SemanticCache::default() }

    fn cache_key(text: &str, state_hint: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let norm: String = text.split_whitespace().take(256).collect::<Vec<_>>().join(" ").to_lowercase();
        let mut h = DefaultHasher::new();
        norm.hash(&mut h);
        state_hint.hash(&mut h);
        h.finish()
    }

    pub fn get(&self, text: &str, state_hint: &str) -> Option<&serde_json::Value> {
        self.cache.get(&Self::cache_key(text, state_hint))
    }

    pub fn set(&mut self, text: &str, value: serde_json::Value, state_hint: &str) {
        self.cache.insert(Self::cache_key(text, state_hint), value);
    }
}

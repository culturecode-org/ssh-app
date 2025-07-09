use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use russh::keys::{PublicKey, Algorithm};

const MAX_LOG_ENTRIES: usize = 1000;

#[derive(Debug, Clone, Default)]
pub struct AuthLog {
    entries: Arc<Mutex<HashSet<String>>>,
}

impl AuthLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn eval_key(&self, key: &PublicKey) -> bool {
        let key_type = key.key_data().algorithm();
        match key_type {
            Algorithm::Ed25519 => true,
            Algorithm::Dsa => false,
            Algorithm::Ecdsa { curve: _ } => false,
            Algorithm::Rsa { hash: _ } => false,
            Algorithm::SkEcdsaSha2NistP256 => false,
            Algorithm::SkEd25519 => false,
            Algorithm::Other(_) => false,
            _ => false
        }
    }

    pub async fn record_key(&self, user: &str, key: &PublicKey) -> bool {
        let fingerprint = key.fingerprint(Default::default()).to_string();
        let entry = format!("{fingerprint}:{user}");

        let mut entries = self.entries.lock().await;
        self.maybe_clear_entries(&mut entries);

        if entries.insert(entry.clone()) {
            log::info!("New public key login attempt: {entry}");
            true // New key, can persist to DB
        } else {
            false // Already logged
        }
    }

    pub async fn all_entries(&self) -> Vec<String> {
        let entries = self.entries.lock().await;
        entries.iter().cloned().collect()
    }

    fn maybe_clear_entries(&self, entries: &mut HashSet<String>) {
        if entries.len() > MAX_LOG_ENTRIES {
            entries.clear();
            log::warn!("Auth log cleared: exceeded {MAX_LOG_ENTRIES} entries.");
        }
    }
}

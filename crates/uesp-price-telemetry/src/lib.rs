use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UespPriceSchema {
    pub frame_id: u64,
    pub fidelity: f64,
    pub entropy: f64,
    pub energy: f64,
    pub position: f64,
    pub previous_hash: String,
    pub ecta_sha256: String,
}

pub struct TelemetryChain {
    pub last_hash: String,
}

impl TelemetryChain {
    pub fn new() -> Self {
        Self {
            last_hash: "a2c95754a5c67fa8d2465b70e125abbc76c6ada11c59c3dd6b9c44a61d20a287"
                .to_string(),
        }
    }

    pub fn record_frame(
        &mut self,
        frame_id: u64,
        fidelity: f64,
        entropy: f64,
        energy: f64,
        position: f64,
    ) -> UespPriceSchema {
        let raw = format!(
            "{}:{}:{}:{}:{}:{}",
            frame_id, fidelity, entropy, energy, position, self.last_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let new_hash = format!("{:x}", hasher.finalize());

        let frame = UespPriceSchema {
            frame_id,
            fidelity,
            entropy,
            energy,
            position,
            previous_hash: self.last_hash.clone(),
            ecta_sha256: new_hash.clone(),
        };

        self.last_hash = new_hash;
        frame
    }
}

impl Default for TelemetryChain {
    fn default() -> Self {
        Self::new()
    }
}

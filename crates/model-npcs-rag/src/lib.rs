use nvidia_nim_microservices::NvidiaNimClient;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelNpcState {
    pub npc_id: String,
    pub quantum_wavefunction_norm: f64,
    pub persona_knowledge_rag: Vec<String>,
}

pub struct ModelNpcRagEngine {
    nim_client: NvidiaNimClient,
}

impl ModelNpcRagEngine {
    pub fn new() -> Self {
        Self {
            nim_client: NvidiaNimClient::new().unwrap(),
        }
    }

    pub async fn query_npc(&self, npc: &ModelNpcState, input_speech: &str) -> String {
        let context = npc.persona_knowledge_rag.join(" | ");
        let prompt = format!(
            "Shinobi Model NPC: {}. Wavefunction Norm: {:.4}. Context: [{}]. Speech: '{}'. Respond in character.",
            npc.npc_id, npc.quantum_wavefunction_norm, context, input_speech
        );

        self.nim_client.generate_wood_style_sdf(&prompt).await.unwrap_or_else(|_| {
            format!("Mokuton Archon Guard [{}]: 'The deep forest responds to your quantum frequency.'", npc.npc_id)
        })
    }
}

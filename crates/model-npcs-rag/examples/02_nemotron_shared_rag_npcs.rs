use model_npcs_rag::{ModelNpcRagEngine, ModelNpcState};

#[tokio::main]
async fn main() {
    println!("=== [Sunshine Engine] Model NPCs Shared RAG Integration ===");

    let rag_engine = ModelNpcRagEngine::new();

    let ninja_npc = ModelNpcState {
        npc_id: "Mokuton_Grandmaster_Archon".into(),
        quantum_wavefunction_norm: 0.999998,
        persona_knowledge_rag: vec![
            "Master of Wood Style Quantum Barriers".into(),
            "Controls 144,000 Cellular L-System Nodes".into(),
            "Powered by NVIDIA Nemotron-4 340B & Parakeet ASR".into(),
        ],
    };

    let speech = "Reveal the Wood Style barrier equations!";
    println!("Player Voice Input: '{}'", speech);

    let response = rag_engine.query_npc(&ninja_npc, speech).await;
    println!("\nNPC Dialogue Response:\n{}", response);
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WoodCellularLSystem {
    pub iteration: u32,
    pub branch_factor: f64,
    pub cell_density: f64,
}

impl WoodCellularLSystem {
    pub fn new(branch_factor: f64) -> Self {
        Self {
            iteration: 0,
            branch_factor,
            cell_density: 1.0,
        }
    }

    pub fn grow_step(&mut self, quantum_density: f64) -> String {
        self.iteration += 1;
        self.cell_density += quantum_density * 0.15 * self.branch_factor;
        format!(
            "sdf::wood_forest(branches={}, density={:.4}, age={})",
            self.iteration * 4,
            self.cell_density,
            self.iteration
        )
    }
}

use serde::{Deserialize, Serialize};
use xeno_ir_parser::{QclAst, QclDialect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestRuthCoefficients {
    pub w1: f64,
    pub w0: f64,
}

impl Default for ForestRuthCoefficients {
    fn default() -> Self {
        let w1 = 1.0 / (2.0 - (2.0f64).powf(1.0 / 3.0));
        let w0 = 1.0 - 2.0 * w1;
        Self { w1, w0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestRuthSubStep {
    pub step_index: usize,
    pub operator: String,
    pub weight: f64,
    pub dt_substep: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticFeedbackResponse {
    pub haptic_amplitude: f32,    // 0.0 to 1.0 scale for rumble motor
    pub trigger_resistance: f32, // For adaptive triggers
    pub is_clipped: bool,        // True if player exceeded stability bounds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledHamiltonianPipeline {
    pub intent_name: String,
    pub wgsl_shader_code: String,
    pub forest_ruth_weights: ForestRuthCoefficients,
    pub symplectic_sequence: Vec<ForestRuthSubStep>,
    pub dt: f64,
    pub dx: f64,
    pub cfl_stable: bool,
    pub stiffness_stable: bool,
    pub max_energy_bound: f64,
}

impl CompiledHamiltonianPipeline {
    /// Computes real-time controller haptics based on the compiled pipeline's
    /// stiffness, energy bounds, and player interaction velocity.
    pub fn compute_controller_haptics(
        &self,
        player_interaction_speed: f64,
    ) -> HapticFeedbackResponse {
        let stiffness_intensity = if self.stiffness_stable {
            (self.max_energy_bound / 500.0).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let raw_amplitude =
            stiffness_intensity * (player_interaction_speed as f32).clamp(0.1, 5.0);

        HapticFeedbackResponse {
            haptic_amplitude: raw_amplitude.min(1.0),
            trigger_resistance: (stiffness_intensity * 0.8)
                + (if !self.cfl_stable { 0.2 } else { 0.0 }),
            is_clipped: !self.cfl_stable || !self.stiffness_stable,
        }
    }
}

pub struct HamiltonianCompiler;

impl HamiltonianCompiler {
    pub fn compile(ast: &QclAst, base_dt: f64) -> CompiledHamiltonianPipeline {
        let mut v_max_limit = 100.0;
        let mut epsilon_val = 0.1;

        for dialect in &ast.dialects {
            match dialect {
                QclDialect::Classical { v_max, epsilon, .. } => {
                    v_max_limit = *v_max;
                    epsilon_val = *epsilon;
                }
            }
        }

        let coeffs = ForestRuthCoefficients::default();
        let dx: f64 = 0.1;
        let dt_cfl: f64 = dx * dx;
        let dt_stiffness: f64 = 1.0 / v_max_limit;
        
        // Physics-aware conditioning & stabilization check
        let stiffness_stable = v_max_limit <= 1000.0 && epsilon_val >= 0.05;
        
        let calculated_dt = dt_cfl.min(dt_stiffness).min(base_dt);
        let dt: f64 = if stiffness_stable {
            calculated_dt
        } else {
            calculated_dt * 0.25
        };

        let cfl_stable = dt <= (dx * dx);
        let max_energy_bound = v_max_limit * 1.5;

        let w1 = coeffs.w1;
        let w0 = coeffs.w0;

        let sequence = vec![
            ForestRuthSubStep {
                step_index: 1,
                operator: "Kinetic".into(),
                weight: w1 * 0.5,
                dt_substep: w1 * 0.5 * dt,
            },
            ForestRuthSubStep {
                step_index: 2,
                operator: "Potential".into(),
                weight: w1,
                dt_substep: w1 * dt,
            },
            ForestRuthSubStep {
                step_index: 3,
                operator: "Kinetic".into(),
                weight: (w1 + w0) * 0.5,
                dt_substep: (w1 + w0) * 0.5 * dt,
            },
            ForestRuthSubStep {
                step_index: 4,
                operator: "Potential".into(),
                weight: w0,
                dt_substep: w0 * dt,
            },
            ForestRuthSubStep {
                step_index: 5,
                operator: "Kinetic".into(),
                weight: (w1 + w0) * 0.5,
                dt_substep: (w1 + w0) * 0.5 * dt,
            },
            ForestRuthSubStep {
                step_index: 6,
                operator: "Potential".into(),
                weight: w1,
                dt_substep: w1 * dt,
            },
            ForestRuthSubStep {
                step_index: 7,
                operator: "Kinetic".into(),
                weight: w1 * 0.5,
                dt_substep: w1 * 0.5 * dt,
            },
        ];

        let wgsl_code = r#"
// Sunshine Engine - Forest-Ruth Order 4 Multi-Step Symplectic Integrator WGSL Shader
struct Params {
    dt: f32,
    substep_weight: f32,
    grid_size: u32,
    v_max: f32,
    epsilon: f32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> psi_real: array<f32>;
@group(0) @binding(2) var<storage, read_write> psi_imag: array<f32>;
@group(0) @binding(3) var<storage, read> potential: array<f32>;

@compute @workgroup_size(128)
fn forest_ruth_potential_kick(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    if (idx >= params.grid_size) { return; }

    let sdf_val = potential[idx];
    let v_reg = params.v_max * tanh(sdf_val / params.epsilon);
    let angle = -v_reg * (params.dt * params.substep_weight);

    let r = psi_real[idx];
    let im = psi_imag[idx];
    
    psi_real[idx] = r * cos(angle) - im * sin(angle);
    psi_imag[idx] = r * sin(angle) + im * cos(angle);
}
"#
        .to_string();

        CompiledHamiltonianPipeline {
            intent_name: ast.intent_name.clone(),
            wgsl_shader_code: wgsl_code,
            forest_ruth_weights: coeffs,
            symplectic_sequence: sequence,
            dt,
            dx,
            cfl_stable,
            stiffness_stable,
            max_energy_bound,
        }
    }
}

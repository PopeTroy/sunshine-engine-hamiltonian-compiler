use serde::{Deserialize, Serialize};
use xeno_ir_parser::QclAst;

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
pub struct CompiledHamiltonianPipeline {
    pub wgsl_shader_code: String,
    pub forest_ruth_weights: ForestRuthCoefficients,
    pub symplectic_sequence: Vec<ForestRuthSubStep>,
    pub dt: f64,
    pub dx: f64,
    pub cfl_stable: bool,
    pub stiffness_stable: bool,
}

pub struct HamiltonianCompiler;

impl HamiltonianCompiler {
    pub fn compile(ast: &QclAst, v_max: f64) -> CompiledHamiltonianPipeline {
        let coeffs = ForestRuthCoefficients::default();
        let dx = 0.1;
        let dt_cfl = dx * dx;
        let dt_stiffness = 1.0 / v_max;
        let dt = dt_cfl.min(dt_stiffness).min(0.002);

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

        let wgsl_code = format!(
            r#"
// Sunshine Engine - Forest-Ruth Order 4 Multi-Step Symplectic Integrator WGSL Shader
struct Params {{
    dt: f32,
    substep_weight: f32,
    grid_size: u32,
    v_max: f32,
    epsilon: f32,
}};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> psi_real: array<f32>;
@group(0) @binding(2) var<storage, read_write> psi_imag: array<f32>;
@group(0) @binding(3) var<storage, read> potential: array<f32>;

@compute @workgroup_size(128)
fn forest_ruth_potential_kick(@builtin(global_invocation_id) id: vec3<u32>) {{
    let idx = id.x;
    if (idx >= params.grid_size) {{ return; }}

    let sdf_val = potential[idx];
    let v_reg = params.v_max * tanh(sdf_val / params.epsilon);
    let angle = -v_reg * (params.dt * params.substep_weight);

    let r = psi_real[idx];
    let im = psi_imag[idx];
    
    psi_real[idx] = r * cos(angle) - im * sin(angle);
    psi_imag[idx] = r * sin(angle) + im * cos(angle);
}}
"#
        );

        CompiledHamiltonianPipeline {
            wgsl_shader_code: wgsl_code,
            forest_ruth_weights: coeffs,
            symplectic_sequence: sequence,
            dt,
            dx,
            cfl_stable: dt <= (dx * dx),
            stiffness_stable: dt <= (1.0 / v_max),
        }
    }
}

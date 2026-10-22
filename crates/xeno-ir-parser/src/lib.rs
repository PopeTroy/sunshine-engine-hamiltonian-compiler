use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum XenoTypeError {
    #[error("Affine Type Violation: Qubit handle '{0}' consumed more than once.")]
    QubitDoubleConsumption(String),
    #[error("Affine Type Violation: Qubit handle '{0}' allocated but never consumed or evolved.")]
    UnconsumedQubitLeak(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QclDialect {
    Ergonomic {
        max_dv_dt: f64,
    },
    Classical {
        sdf_expression: String,
        v_max: f64,
        epsilon: f64,
    },
    Quantum {
        dt: f64,
        trotter_order: u8,
        kappa: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QclAst {
    pub intent_name: String,
    pub dialects: Vec<QclDialect>,
    pub allocated_qubits: HashSet<String>,
    pub consumed_qubits: HashSet<String>,
}

pub struct AffineTypeChecker;

impl AffineTypeChecker {
    pub fn validate(ast: &QclAst) -> Result<(), XenoTypeError> {
        let mut consumed = HashSet::new();
        for qbit in &ast.consumed_qubits {
            if !consumed.insert(qbit.clone()) {
                return Err(XenoTypeError::QubitDoubleConsumption(qbit.clone()));
            }
        }

        for qbit in &ast.allocated_qubits {
            if !ast.consumed_qubits.contains(qbit) {
                return Err(XenoTypeError::UnconsumedQubitLeak(qbit.clone()));
            }
        }

        Ok(())
    }
}

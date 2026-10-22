# Sunshine Engine: Xeno-IR v3.1 Hamiltonian Compiler & Wood-Style Quantum Mechanics Engine

[![ECTA Dilation Manifest](https://img.shields.io/badge/ECTA_Manifest-SHA256_a2c95754...-blueviolet)](https://celsiustechmediagroup.co.za)
[![TRL Level](https://img.shields.io/badge/TRL-9_Production_Harden-brightgreen)]()
[![NVIDIA NIM Active](https://img.shields.io/badge/NVIDIA_NIM-Nemotron_Parakeet_Shared_RAG-76B900?logo=nvidia)](https://build.nvidia.com)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 📌 Repository Metadata
* **Repo Name:** `sunshine-engine-hamiltonian-compiler`
* **Description:** TRL-9 Production Hamiltonian Compiler for the Sunshine Engine. Integrates Xeno-IR v3.1, Forest-Ruth 4th-Order Symplectic Integration, Wood-Style (Mokuton) Quantum Cellular Growth Dynamics, and NVIDIA NIM Microservices (Nemotron-4 340B, Parakeet ASR, NV-Embed-QA RAG).
* **Maintainer:** Celsius Tech Media Group (`info@celsiusmediagroup.co.za`)

---

## 🪵 Scientific Mechanics: Wood Style (Mokuton) Quantum Dynamics
In the Sunshine Engine, **Wood Style (Mokuton)** is formalized as a time-dependent non-linear reactive potential field $V_{\text{Wood}}(x, t)$ coupled to cellular fluid transport (dermatogen/vascular cambium expansion dynamics).

### Mathematical Formulation
$$V_{\text{Wood}}(x, t) = V_{\max} \cdot \tanh\left( \frac{\text{SDF}_{\text{L-System}}(x, t)}{\varepsilon} \right) + \gamma \int_0^t \vert{}\Psi(x, \tau)\vert{}^2 d\tau$$

1. **L-System Branching Morphogenesis:** Generates self-similar cellular geometry represented as continuous Signed Distance Fields.
2. **Probability-Density Feedback ($\gamma$):** Wavefunction intensity $\vert{}\Psi\vert{}^2$ accelerates local cellular growth, allowing quantum observation to physically manifest structural wood barriers in real time.
3. **Regularization:** Hyperbolic tangent $\tanh(\cdot / \varepsilon)$ prevents infinite potential steps, ensuring continuous $C^2$ differentiability required by 4th-order integrators.

---

## 🔬 Symplectic Mechanics: True Forest-Ruth 4th-Order Integrator

The Hamiltonian operator $\hat{H} = \hat{T} + \hat{V}$ is evolved via the 4th-order Suzuki-Yoshida composition:

$$\hat{U}(\Delta t) = e^{-i(\hat{T}+\hat{V})\Delta t} \approx S_2(w_1 \Delta t) \cdot S_2(w_0 \Delta t) \cdot S_2(w_1 \Delta t) + \mathcal{O}(\Delta t^5)$$

Where $S_2(\delta t) = e^{-i\hat{T}\frac{\delta t}{2}} e^{-i\hat{V}\delta t} e^{-i\hat{T}\frac{\delta t}{2}}$ (Strang Splitting), and the Suzuki weights are:

$$w_1 = \frac{1}{2 - 2^{1/3}} \approx 1.3512071919596576, \quad w_0 = 1 - 2w_1 \approx -1.7024143839193155$$

---

## 🚀 Quick Start & Environment Execution

### 1. Set NVIDIA NIM API Secrets
```bash
export NVIDIA_API_KEY="nvapi-YOUR_NVIDIA_API_KEY_HERE"
export NVIDIA_NIM_BASE_URL="[https://integrate.api.nvidia.com/v1](https://integrate.api.nvidia.com/v1)"

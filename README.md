# Internal Developer Platform (IDP)
# 🚀 IDP Platform — Internal Developer Platform

#DEVOPS #INFRASTRUCTURE #CLOUD

[![CI Rust](https://github.com/your-org/idp-platform/actions/workflows/ci-rust.yml/badge.svg)](https://github.com/your-org/idp-platform/actions/workflows/ci-rust.yml)
[![CI Go](https://github.com/your-org/idp-platform/actions/workflows/ci-go.yml/badge.svg)](https://github.com/your-org/idp-platform/actions/workflows/ci-go.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Go Report Card](https://goreportcard.com/badge/github.com/your-org/idp-platform)](https://goreportcard.com/report/github.com/your-org/idp-platform)

> **Deploy applications in seconds, not days.**  
> A self-service Internal Developer Platform built with Rust, Go, Python, Kubernetes, Terraform, and Ansible.

---

## 📖 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Components](#components)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

IDP Platform empowers developers to deploy, manage, and monitor their applications
autonomously without requiring infrastructure knowledge. It provides:

- 🛠️ **CLI tool** (`idpctl`) for seamless developer experience
- ⚙️ **Control Plane API** for orchestration and provisioning
- 💰 **ML Cost Engine** for cost prediction and anomaly detection
- 🏗️ **IaC** with Terraform + Ansible
- 🚢 **GitOps** with ArgoCD on K8s/K3s

---

## Architecture

```mermaid
graph TB
    Dev[👨‍💻 Developer] -->|idpctl deploy| CLI[🦀 Rust CLI - idpctl]
    CLI -->|REST/gRPC| CP[🐹 Go Control Plane]
    CP -->|K8s API| K8S[☸️ Kubernetes / K3s]
    CP -->|Predict Cost| ML[🐍 Python ML Engine]
    CP -->|Auth| AUTH[🔐 Auth Service]
    ML -->|Store| TSDB[(📊 TimescaleDB)]
    K8S -->|Metrics| OTEL[📡 OpenTelemetry Collector]
    OTEL -->|Metrics| PROM[📈 VictoriaMetrics]
    OTEL -->|Logs| LOKI[📋 Loki]
    OTEL -->|Traces| TEMPO[🔍 Tempo]
    PROM --> GRAF[📊 Grafana]
    LOKI --> GRAF
    TEMPO --> GRAF
    ARGO[🔄 ArgoCD] -->|GitOps| K8S


    
## Architecture Decision Records (ADR)

This repository contains the source code and configuration for the Internal Developer Platform.

### Project Structure
- `docs/`: General documentation and architecture diagrams.
- `cli/`: Rust-based CLI for developer interaction (`idpctl`).
- `control-plane/`: Go-based management API.
- `ml-cost-engine/`: Python-based ML engine for cost prediction and anomaly detection.
- `infra/`: Terraform and Ansible configurations.
- `k8s/`: Kubernetes manifests and GitOps configuration.
- `.github/`: CI/CD pipelines.

### Architecture Decision Records
ADRs are stored in [docs/adr/](docs/adr/).
- [ADR-001: Technology Stack Selection](docs/adr/001-tech-stack.md)




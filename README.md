# Platform-Engineering-Self-Service-IDP
Une plateforme interne permettant aux devs de déployer leurs apps en autonomie via une UI/CLI, sans toucher à l'infra


# Internal Developer Platform (IDP)

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

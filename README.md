# helixVision

| | |
|---|---|
| **Version** | 0.1.0 (seed) |
| **Status** | Architectural draft — seed CLI only |
| **Rust files** | 1 (`src/main.rs`) |
| **MSRV** | 1.87 (edition 2024) |
| **License** | AGPL-3.0 |
| **Unsafe** | `#![forbid(unsafe_code)]` |
| **C deps** | Zero (ecoBin compliant) |
| **Primals consumed** | 11 domains (security, discovery, compute, shader, math, storage, dag, ledger, attribution, ai, visualization) |
| **Source springs** | wetSpring (genomics), neuralSpring/coralForge (structure), hotSpring (MD validation) |
| **Last validation** | N/A (pre-alpha) |

**A [sporeGarden](https://github.com/sporeGarden) project — sovereign genomics discovery on your own hardware.**

helixVision is not a spring. It is a standalone sporeGarden product that composes
validated science from wetSpring (16S/WGS genomics, 6656+ checks),
neuralSpring/coralForge (AlphaFold structure prediction, 154 checks), and
hotSpring (MD validation) into a deployable pipeline via BYOB composition.

## Key Capability

**Sovereign 16S/WGS pipeline** — no QIIME2, no EPI2ME, no Python runtime.
From MinION sequencer to taxonomy assignment to community profile on sovereign
hardware.

## Relationship to Siblings

| Product | Domain | Source Spring | Status |
|---------|--------|--------------|--------|
| **esotericWebb** | Narrative CRPG | ludoSpring | V6, 342 tests |
| **initioChem** | Computational chemistry | hotSpring | Seed |
| **blueFish** | Analytical chemistry ETL | TBD | Pending |
| **helixVision** | Genomics discovery | wetSpring + neuralSpring + hotSpring | Seed (this) |

All sporeGarden products follow the same architectural pattern:
- Consume primals via JSON-RPC IPC (zero crate deps on springs)
- Discover capabilities via Songbird at runtime
- Degrade gracefully when primals are absent
- Feed gaps back into the ecosystem

## coralForge Relationship

coralForge (gen3 science in neuralSpring) becomes helixVision's structure
prediction module. The science stays in neuralSpring; helixVision is the gen4
product layer.

## Architecture

helixVision sits atop the primal stack as a **genomics discovery engine**.
It consumes primals via JSON-RPC IPC — zero Rust crate dependencies on any
spring. Primals are resolved from `plasmidBin/` or discovered via Songbird
at runtime.

```
Springs (science + experiments)  →  produce  →  primals (genomeBin/ecoBin)
                                                       ↓
                                               plasmidBin/ (deployment)
                                                       ↓
                                      helixVision discovers + composes via IPC
```

| Domain | Primal | Role | Status |
|--------|--------|------|--------|
| security | BearDog | Sample signing, genomic vault encryption | Planned |
| discovery | Songbird | Federation between field station and lab | Planned |
| compute | ToadStool | GPU dispatch for structure prediction, diversity | Planned |
| shader | CoralReef | WGSL shader compilation (sovereign GPU stack) | Planned |
| math | BarraCuda | f64 math primitives, alignment acceleration | Planned |
| storage | NestGate | FASTA/FASTQ storage, Genomic Vault | Planned |
| dag | RhizoCrypt | Analysis session DAG (sample → results) | Planned |
| ledger | LoamSpine | Sample certificates, chain-of-custody | Planned |
| attribution | SweetGrass | Collector, analyst, publisher attribution | Planned |
| ai | Squirrel | AI-assisted community profile interpretation | Planned |
| visualization | petalTongue | Community profiles, 3D structures, dashboards | Planned |

## Faculty Contacts

- Rika Anderson (Carleton) — QS framework, soil microbiome
- Waters (MSU) — wastewater monitoring, AMR
- Cahill (Sandia) — environmental genomics

## Applications

1. **Field Genomics** — MinION + laptop, sovereign on-site pipeline
2. **Wastewater Monitoring** — AMR genes, pathogen emergence, PFAS-degrading organisms
3. **Harmful Algal Bloom Detection** — eDNA sentinel for Microcystis
4. **LTEE Structural Analysis** — 75,000 generation protein fold evolution
5. **Soil Microbiome** — Inoculant effectiveness, Anderson QS framework

## Deploy Graph Tiers

| Tier | Graph | Primals | Use Case |
|------|-------|---------|----------|
| 1 | `helixvision_tower.toml` | BearDog + Songbird | Secure transport, signed results |
| 2 | `helixvision_sequence.toml` | + ToadStool + CoralReef + BarraCuda | GPU structure prediction, large-scale diversity |
| 3 | `helixvision_provenance.toml` | + RhizoCrypt + LoamSpine + SweetGrass | Sample-to-publication chain-of-custody |
| 4 | `helixvision_field.toml` | + federation | Field deployment with crypto signing and local mesh |
| 5 | `helixvision_full.toml` | + NestGate + Squirrel + petalTongue + helixVision | Complete pipeline |

## Graceful Degradation

| Tier | Requires | Capability |
|------|----------|------------|
| Standalone | Nothing | FASTQ → taxonomy assignment using local databases (CSV output) |
| Minimal | Core only | 16S pipeline + diversity metrics (text output) |
| Compute | + toadStool/barraCuda | GPU structure prediction, large-scale diversity analysis |
| Provenance | + trio | Sample-to-publication chain-of-custody |
| Full | All | Structure prediction, community profiling, provenance, visualization, AI, federation |

## Quick Start

```bash
cargo build --release
./target/release/helixvision status
./target/release/helixvision capabilities
./target/release/helixvision classify /path/to/sample.fastq

# Full deployment (future):
biomeos deploy --graph graphs/helixvision_full.toml
```

## Directory Structure

```
helixVision/
├── README.md              # This file
├── Cargo.toml             # Workspace definition
├── deny.toml              # ecoBin C-dep bans
├── LICENSE                # AGPL-3.0
├── graphs/                # biomeOS deploy graphs (5 tiers)
│   ├── helixvision_tower.toml
│   ├── helixvision_sequence.toml
│   ├── helixvision_provenance.toml
│   ├── helixvision_field.toml
│   └── helixvision_full.toml
├── niches/                # biomeOS niche YAML
│   └── helixvision.yaml
├── specs/                 # Product specifications
│   └── PRODUCT_SPEC.md
└── src/                   # Product binary (seed CLI)
    ├── Cargo.toml
    └── main.rs
```

## References

- [sporeGarden org](https://github.com/sporeGarden)
- Remote: `git@github.com:sporeGarden/helixVision.git`
- Source spring (primary): `ecoPrimals/springs/wetSpring/`
- Structure prediction: `ecoPrimals/springs/neuralSpring/` (coralForge)
- MD validation: `ecoPrimals/springs/hotSpring/`
- Complements: `gardens/initioChem/`, blueFish (pending)
- Reference pattern: `gardens/esotericWebb/`, `gardens/initioChem/`

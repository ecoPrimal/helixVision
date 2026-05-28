# helixVision — Product Specification

**Version:** 0.1.0 (seed)
**Status:** Architectural draft — composition contract defined
**Pattern:** sporeGarden gen4 (BYOB, PrimalBridge, graceful degradation)
**Domain:** Sovereign genomics discovery — field to publication
**Remote:** git@github.com:sporeGarden/helixVision.git

---

## Product Identity

| Field | Value |
|-------|-------|
| **Name** | helixVision |
| **Org** | [sporeGarden](https://github.com/sporeGarden) |
| **Source springs** | wetSpring (primary), neuralSpring/coralForge (structure), hotSpring (MD) |
| **Domain** | Field genomics, wastewater monitoring, eDNA, LTEE structural analysis, HAB detection |
| **Audience** | Field biologists, environmental monitoring labs, structural genomics researchers |
| **Complements** | initioChem (CompChem), blueFish (analytical chemistry ETL), esotericWebb (CRPG) |
| **Pattern analog** | esotericWebb : ludoSpring :: helixVision : wetSpring |

## What It Does

A sovereign genomics discovery platform — from biological sample to scientific
understanding on your own hardware:

1. Import FASTQ from MinION or file (`genomics.sample.ingest`)
2. Assign 16S taxonomy without QIIME2 or Python (`genomics.16s.classify`)
3. Assemble WGS contigs on sovereign hardware (`genomics.wgs.assemble`)
4. Profile community diversity — Shannon, Simpson (`genomics.diversity.profile`)
5. Predict protein structures via coralForge/AlphaFold (`genomics.structure.predict`)
6. Detect community shifts with ESN classifier (`genomics.anomaly.detect`)
7. Seal sample-to-publication provenance (`genomics.provenance.seal`)
8. Monitor sentinels for PFAS, HAB, AMR (`genomics.sentinel.monitor`)

## Why It Exists

The science is validated (wetSpring 6656+ checks, coralForge 154 checks).
What's missing: **a sovereign product layer** that composes genomics, structure
prediction, and MD validation into one deployable pipeline — no cloud dependency,
no Python runtime, no QIIME2 lock-in.

---

## Composition Contract

| Property | Value |
|----------|-------|
| **Particle profile** | electron-heavy (GPU structure + CPU alignment + NPU edge) |
| **Mixed atomic** | sequence + tower (GPU compute + secure transport) |
| **Bonding** | ionic lease (compute sessions) + covalent (persistent provenance) |
| **Degradation** | 5 tiers (standalone → minimal → compute → provenance → full) |

### Source Spring Responsibilities

| Spring | Contribution | Checks |
|--------|-------------|--------|
| wetSpring | 16S/WGS genomics, taxonomy, diversity | 6656+ |
| neuralSpring/coralForge | AlphaFold structure prediction | 154 |
| hotSpring | MD validation of predicted structures | per hotSpring |

The product binary orchestrates these — it does not re-implement alignment,
assembly, or structure prediction.

## Deploy Graphs

Five tiered graphs in `graphs/`, following the initioChem/esotericWebb pattern:

| Graph | Tier | What it enables |
|-------|------|-----------------|
| `helixvision_tower.toml` | 1 | Secure transport, signed results |
| `helixvision_sequence.toml` | 2 | GPU structure prediction, large-scale diversity |
| `helixvision_provenance.toml` | 3 | Sample-to-publication chain-of-custody |
| `helixvision_field.toml` | 4 | Field deployment with federation and local mesh |
| `helixvision_full.toml` | 5 | Storage, AI interpretation, visualization, product binary |

## Evolution Roadmap

| Phase | Target | Blocks On |
|-------|--------|-----------|
| 0.2 | Standalone 16S classify (local DB, CSV output) | wetSpring taxonomy module |
| 0.4 | WGS assembly + diversity metrics | barraCuda alignment acceleration |
| 0.6 | Structure prediction via coralForge | toadStool GPU dispatch, coralReef WGSL |
| 0.8 | Provenance sealing + field federation | provenance trio, Songbird mesh |
| 1.0 | Sentinel monitoring + AI interpretation | Squirrel community analysis, petalTongue dashboards |

## Upstream Dependencies

| What | Owner | Status |
|------|-------|--------|
| wetSpring 16S pipeline | wetSpring team | Validated (6656+ checks) |
| coralForge AlphaFold | neuralSpring team | Validated (154 checks) |
| hotSpring MD validation | hotSpring team | Available |
| petalTongue genomics viz | petalTongue team | Specified, not implemented |
| barraCuda alignment primitives | barraCuda team | Partial |
| biomeOS `nucleus ingest` CLI | biomeOS team | NC-1 gate |

## References

- Source spring (primary): `ecoPrimals/springs/wetSpring/`
- Structure prediction: `ecoPrimals/springs/neuralSpring/` (coralForge)
- MD validation: `ecoPrimals/springs/hotSpring/`
- Complements: `gardens/initioChem/`, blueFish (pending)
- Reference pattern: `gardens/esotericWebb/`, `gardens/initioChem/`
- Composition onramp: `infra/wateringHole/GARDEN_COMPOSITION_ONRAMP.md`

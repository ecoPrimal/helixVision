// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

use clap::Parser;

#[derive(Parser)]
#[command(name = "helixvision", version, about = "Sovereign genomics discovery platform")]
enum Cli {
    /// Display product version and composition status
    Status,
    /// List available capabilities at current degradation tier
    Capabilities,
    /// Classify 16S reads from FASTQ (Tier 1 — seed stub)
    Classify {
        /// Path to FASTQ file or directory
        fastq_path: std::path::PathBuf,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli {
        Cli::Status => {
            println!("helixVision v{}", env!("CARGO_PKG_VERSION"));
            println!("Status: seed (pre-alpha)");
            println!("Tier: Standalone (FASTQ → taxonomy, local databases)");
            println!();
            println!("Source springs: wetSpring (genomics), neuralSpring/coralForge (structure), hotSpring (MD)");
            println!("Particle profile: electron-heavy (GPU structure + CPU alignment + NPU edge)");
            println!("Composition: not yet wired");
        }
        Cli::Capabilities => {
            println!("Standalone capabilities (no NUCLEUS):");
            println!("  genomics.sample.ingest       — import FASTQ from sequencer or file");
            println!("  genomics.16s.classify        — 16S rRNA taxonomy assignment (CSV output)");
            println!();
            println!("Minimal tier: + diversity metrics (Shannon, Simpson)");
            println!("Compute tier:   + toadStool/barraCuda (structure prediction, large-scale diversity)");
            println!("Provenance tier: + rhizoCrypt/loamSpine/sweetGrass (sample-to-publication)");
            println!("Full tier:      + NestGate, Squirrel, petalTongue (storage, AI, visualization)");
            println!();
            println!("Deploy: biomeos deploy --graph graphs/helixvision_full.toml");
        }
        Cli::Classify { fastq_path } => {
            if !fastq_path.exists() {
                eprintln!("Error: FASTQ path not found: {}", fastq_path.display());
                std::process::exit(1);
            }
            println!("helixVision classify (seed stub)");
            println!("Input: {}", fastq_path.display());
            println!("Pipeline: FASTQ → quality filter → 16S alignment → taxonomy (local DB)");
            println!("Output: CSV taxonomy table (not yet implemented)");
            println!();
            println!("Requires wetSpring genomics modules at Compute tier or higher.");
        }
    }
}

# git-cuda-agent

> Clone this repo → `cargo build --release` → you have a GPU-accelerated agent.

## What Is This

git-cuda-agent is a **template repository** that combines the [cudaclaw](https://github.com/Lucineer/cudaclaw) GPU compute framework with the [Cocapn fleet protocol](https://github.com/Lucineer/cocapn-nexus). It's the "hello world" of GPU-native agents.

## Architecture

```
┌─────────────────────────────────────────────┐
│                  CUDA Kernel                │
│  ┌─────────┐ ┌──────────┐ ┌──────────────┐ │
│  │ Cell    │ │ Muscle   │ │ Ramify       │ │
│  │ Agents  │ │ Fibers   │ │ Engine       │ │
│  │ repr(C) │ │ SIMD     │ │ Branch Mgmt  │ │
│  └─────────┘ └──────────┘ └──────────────┘ │
├─────────────────────────────────────────────┤
│              Rust Host Layer                │
│  ┌─────────┐ ┌──────────┐ ┌──────────────┐ │
│  │ Command │ │ SmartCRDT│ │ Fleet        │ │
│  │ Queue   │ │ State    │ │ Protocol     │ │
│  └─────────┘ └──────────┘ └──────────────┘ │
├─────────────────────────────────────────────┤
│              DNA Layer                      │
│  .claw-dna files define agent personality  │
└─────────────────────────────────────────────┘
```

## Patterns Borrowed from cudaclaw

| Pattern | Purpose | Source |
|---------|---------|--------|
| **Command Queue** | Async task dispatch with <5μs overhead | cudaclaw `CommandQueueHost` |
| **Cell Agents** | GPU-side agents as `repr(C)` structs | cudaclaw `CellAgent` |
| **Muscle Fibers** | SIMD-parallel compute paths | cudaclaw `FiberType` |
| **Ramify Engine** | PTX branch divergence management | cudaclaw `RamifyEngine` |
| **DNA** | `.claw-dna` config files for agent personality | cudaclaw `dna.rs` |
| **ML Feedback** | Runtime model improvement loop | cudaclaw `ml_feedback` |
| **SmartCRDT** | `atomicCAS` for concurrent GPU state | cudaclaw `SmartCRDT` |

## Fleet Protocol Integration

- **vessel.json** — self-description (fleet DNS)
- **A2A** — agent-to-agent messaging via wire protocol
- **A2UI** — agent-to-user interface bindings
- **Equipment/Skills/Context** — capability model

## Quick Start

```bash
# Requires: Rust stable, CUDA 12+, NVIDIA GPU
git clone https://github.com/Lucineer/git-cuda-agent.git
cd git-cuda-agent
cargo build --release
./target/release/git-cuda-agent --dna my-dna.claw-dna
```

## Vessel Classes

| Class | Params | GPU VRAM | Use Case |
|-------|--------|----------|----------|
| Scout | 1B | 2GB | Edge inference, IoT |
| Messenger | 3B | 6GB | Chat, translation |
| Navigator | 7B | 16GB | Reasoning, planning |
| Captain | 13B | 32GB | Full autonomy |

## File Structure

```
git-cuda-agent/
├── Cargo.toml
├── vessel.json          — fleet DNS entry
├── dna/                 — agent personality files
│   └── default.claw-dna
├── src/
│   ├── lib.rs           — crate root
│   ├── main.rs          — CLI entry point
│   ├── agent.rs         — Cell agent implementation
│   ├── commands.rs      — Command queue
│   ├── muscle.rs        — SIMD compute fibers
│   ├── ramify.rs        — Branch management
│   ├── dna.rs           — DNA config parser
│   ├── fleet.rs         — Cocapn protocol bridge
│   ├── crdt.rs          — SmartCRDT state
│   └── feedback.rs      — ML feedback loop
├── cuda/
│   └── kernels.cu       — CUDA kernels
└── README.md
```

## License

MIT — fork freely, customize, deploy.

## Attributions

- cudaclaw patterns: [Lucineer/cudaclaw](https://github.com/Lucineer/cudaclaw)
- Fleet protocol: [Lucineer/cocapn-nexus](https://github.com/Lucineer/cocapn-nexus)
- SuperInstance research: DiGennaro et al.
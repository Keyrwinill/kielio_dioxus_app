# Board game: Dead Man's Draw

## Overview

A Rust implementation of *Dead Man's Draw* with a shared game engine, Dioxus frontend, AI opponents, and variant support.

## Features

- 2–4 player support
- Human and AI players
- Official setup with initial discard pile
- Core suit abilities
- Mermaid Variant
- Shared Rust game engine
- Web/Desktop frontend with Dioxus
- Tested game logic

## Variants

### Base Game

Uses the standard Mermaid card values:

- Mermaid: 4–9

### Mermaid Variant

Uses low-value Mermaid cards:

- Mermaid: 2–7

When a Mermaid is played, choose a non-Mermaid card already in the play area. That card is moved next to the Mermaid and its suit ability is activated again.

## Technologies
- Rust
- Dioxus
- Tailwind CSS


## Run

Backend:

cargo run -p backend

Frontend:
dx serve -p frontend

Test:
cargo test -p shared


## Screenshots

- Board game home page
  ![Home page](screenshots/(2026-07-12)home_page.png "Home page")
- Dead Man's Draw: setup
  ![Dead Man's Draw: setup](screenshots/(2026-07-12)dead_mans_draw_setup.png "Dead Man's Draw: setup")
- Dead Man's Draw: gameplay
  ![Dead Man's Draw 1](screenshots/(2026-07-12)dead_mans_draw_1.png "Dead Man's Draw 1")
  ![Dead Man's Draw 2](screenshots/(2026-07-12)dead_mans_draw_2.png "Dead Man's Draw 2")


## Future Work

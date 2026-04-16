# Tetris

A classic Tetris game built in Rust with [macroquad](https://github.com/not-fl3/macroquad).

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- **SRS (Super Rotation System)** with full wall kick tables
- **7-bag randomizer** for fair piece distribution
- **Ghost piece** preview
- **Hold piece** system
- **5-piece next queue** with fade effect
- **DAS/ARR** for responsive movement
- **Lock delay** with reset on move/rotate
- **Scoring** with back-to-back Tetris bonus and combo system
- **Line clear animations** with particles and screen shake
- **Pause** support

## Controls

| Key | Action |
|---|---|
| Left / Right / A / D | Move |
| Up / W | Rotate CW |
| Z | Rotate CCW |
| Down / S / Space | Hard Drop |
| C / Shift | Hold |
| P / ESC | Pause |
| Enter | Start / Restart |

## Build & Run

```sh
cargo run
```

For a release build with optimizations:

```sh
cargo run --release
```

## License

[MIT](LICENSE)

# wlpick

A Wayland color picker with a magnifying loupe. Captures the screen, lets you zoom in on a pixel, and copies the color to your clipboard on click. Inspired by Firefox's Eyedropper tool.

## Features

- Fullscreen overlay with live magnifying loupe
- Crosshair for precise pixel targeting
- Color swatch preview with hex value
- Multiple output formats: `hex`, `rgb`, `hsl`
- Copies result to clipboard automatically
- Prints to stdout (pipeable)

## Requirements

- A Wayland compositor with `wlr-screencopy` support (Sway, Hyprland, and most wlroots-based compositors)
- [`grim`](https://sr.ht/~emersion/grim/) — for screen capture
- [`wl-clipboard`](https://github.com/bugaevc/wl-clipboard) — for clipboard support

```bash
sudo apt install grim wl-clipboard  # Debian/Ubuntu
```

## Installation

### From source

```bash
git clone https://github.com/BarriosXJavier/wlpick
cd wlpick
cargo install --path .
```

Requires [Rust](https://rustup.rs) 1.70+. The binary lands in `~/.cargo/bin/wlpick`.

## Usage

```bash
wlpick                     # default: hex output, copies to clipboard
wlpick --format rgb        # output as rgb(r, g, b)
wlpick --format hsl        # output as hsl(h, s%, l%)
wlpick --no-clipboard      # print only, skip clipboard
```

### Keybinds

| Action | Input |
|--------|-------|
| Pick color | Left click |
| Cancel | Close window / `Escape` |

### Piping

Since the color is printed to stdout, you can pipe it:

```bash
# Use picked color in a script
COLOR=$(wlpick)
echo "You picked: $COLOR"

# Notify via dunst
wlpick | xargs -I{} notify-send "Picked color" {}
```

## Output formats

| Format | Example |
|--------|---------|
| `hex` (default) | `#1A2B3C` |
| `rgb` | `rgb(26, 43, 60)` |
| `hsl` | `hsl(210, 40%, 17%)` |

## Project structure

```
wlpick/
├── src/
│   ├── main.rs        # CLI parsing, entry point
│   ├── screenshot.rs  # screen capture via grim
│   ├── loupe.rs       # magnifier UI via raylib
│   ├── color.rs       # color formatting and HSL conversion
│   └── clipboard.rs   # clipboard via wl-copy
└── Cargo.toml
```

## License

GPL-3.0


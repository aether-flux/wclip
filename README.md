# WireClip

**Zero-config, privacy-first local network clipboard synchronization daemon in Rust.**

`WireClip` is a lightweight background service that seamlessly syncs text between your desktop workstation and mobile devices over your local Wi-Fi network. Built with an earthy, warm minimalist interface, it keeps your sensitive clipboard data strictly inside your local subnet—no cloud services, no third-party accounts, and zero tracking.

---

## Features

- **Instant Two-Way Sync:** Text copied on desktop streams to your phone; text sent from your phone is instantly written to your desktop clipboard.
- **Wayland Native & Cross-Platform:** Direct support on Linux (Wayland/X11), macOS, and Windows via `arboard`.
- **Low Latency:** Real-time bi-directional streaming powered by WebSockets.
- **PWA Ready:** Clean, warm-toned web interface optimized for mobile browsers with HTTP fallback for secure clipboard copying on local IP addresses.
- **Single Binary:** Everything, including the frontend assets, is compiled into a single lightweight Rust executable.

---

## Prerequisites

### Linux (Wayland)
For instant event-driven background sync on Wayland compositors (Hyprland, Sway, GNOME, KDE):
```bash
# Arch Linux
sudo pacman -S wl-clipboard

# Ubuntu / Debian
sudo apt install wl-clipboard
```

### Linux (X11) / Windows / MacOS
No extra tools required. `WireClip` uses native system APIs via `arboard`.

## Installation

### Cargo
```sh
cargo install wclip
```

### Manual Build
```sh
# Clone the repo
git clone https://github.com/aether-flux/wclip
cd wclip

# Build the binary
cargo build --release

# Run the binary
./target/releases/wclip
```

---

## Usage

1. Start clp on your main computer:
    ```sh
    wclip
    ```
2. The terminal will output your local IP address and port (e.g., `http://192.168.1.50:8080`).
3. Open the printed URL on your phone or tablet connected to the same Wi-Fi network.
4. **Desktop -> Phone**: Copy any text on your desktop. It will automatically populate on your mobile screen. Tap Copy to Phone Clipboard.
5. **Phone -> Desktop**: Type or paste text into the input field on your phone and tap Send to Desktop. It is now on your desktop clipboard ready to paste (`Ctrl+V / Cmd+V`).

---

## License
MIT

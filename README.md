# Gyrscale

A lightweight system watcher, watchdog, and Fastfetch-style fetcher built in Rust. 

### Pronunciation
**Gyrscale** is named after the *gyrfalcon* (the fast bird of prey) + *scale*.
It is pronounced **"Jer-skayl"** (rhymes with *sir-scale*).

---

### Why do we need another fetch utility?

On the surface, this might look like just another Neofetch or Fastfetch clone, but it’s something more. Gyrscale is a tool born entirely out of sheer, unadulterated debugging frustration. 

It all started when my system began to quietly fall apart. First, my browser's UI started failing: WebGL stopped working, icons refused to render, and eventually, I couldn't even edit URLs without opening a brand-new tab. Then I noticed that if I let my system sit at the `greetd` login screen for too long, it would straight-up refuse to launch my compositor (`niri`) and just throw errors. 

After an extensive debugging deep-dive, I discovered a perfect storm of hardware and software chaos:

* **The Kernel Bug:** A recent NixOS flake update had bumped my Zen kernel, introducing an `amdgpu` bug that was silently wrecking my GUI. Swapping the Zen kernel out for `mainline_latest` fixed the UI issues, but the rabbit hole went deeper.
* **The Dying Mouse:** My 16-month-old, literal one-dollar mouse from China (which didn't even have a vendor ID) was taking 30 seconds just to respond to the system. A quick check of `journalctl` revealed a barrage of USB protocol handshake failures (error -71). The mouse was actively poisoning the kernel.
* **The Ghost Dongle:** For the past two years, my ethernet connection would randomly die for exactly 30 or 60 seconds. I always assumed it was a bad cable and replaced it, to no avail. As it turned out, a no-name Chinese 2.4G Wi-Fi dongle had been plugged into the back of my PC for years, forgotten. The moment I unplugged it, every single remaining system hitch vanished.

That is the story behind **Gyrscale**. It exists because a standard fetch utility only tells you what your system *looks* like on paper. Gyrscale is built to tell you when your system is secretly drowning in hardware errors before they ruin your workflow.

---

### Technical Approach
To keep things lightweight and fast, the goal is to live as close to the system as possible. I am using the `sysinfo` crate for low-overhead, direct hardware and resource tracking, minimizing external abstractions to keep memory and CPU usage minimal.

---

### Roadmap

Since I am learning Rust while building this, the project is broken down into three distinct phases:

#### Phase 1: The Core Fetcher (Months 1–3)
* Focus on learning Rust fundamentals (borrowing, memory management, crate ecosystems).
* Build a standard, blazing-fast Fastfetch-style system information fetcher using `sysinfo`.
* Parse basic system metrics (CPU, OS, Kernel, Uptime, Memory) and print them cleanly to stdout.

#### Phase 2: The TUI Layer (Months 4–7)
* Integrate `ratatui` to build a terminal user interface.
* Implement an `rmpc`-style tabbed layout to seamlessly switch views.
* **Tab 1:** The Phase 1 system fetcher view.
* **Tab 2:** A `btop`-style live resource monitor (real-time CPU graphs, memory pressure, process lists via `sysinfo`).

#### Phase 3: The Watchdog (Months 8–10)
* Turn Gyrscale into an active system watchdog.
* Implement background monitoring for kernel rings and `journalctl`.
* Set up real-time alerting for silent hardware/driver failures (like the USB protocol error -71 or ghost network drops that inspired this project).

---
### Building

```bash
cargo build --release
```

## License
`MIT`

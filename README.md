# The Sanjay Times — WebAssembly Portfolio

A vintage-style newspaper portfolio application engineered using Rust, Yew, WebAssembly, and Tailwind CSS. This system replicates a traditional print publication layout to present technical credentials, project disclosures, and research highlights in a unique, retro-themed digital format.

---

## Technical Architecture

The codebase leverages a modern Rust-to-Wasm compiler pipeline for high performance, type safety, and efficient bundle sizes:

* **Framework:** Yew (v0.21+ / Git CSR version) — A component-based Rust framework for compiling client-side user interfaces to WebAssembly.
* **Styling Paradigm:** Double-column modular styles using `stylist` (CSS-in-Rust) for vintage newspaper textures and layout alignments, combined with Tailwind CSS for utility grids.
* **Bundler & Tooling:** Trunk — A Rust-powered build tool and asset bundler designed to compile the application and copy static resources into a production-ready assembly.
* **Libraries:**
  * `chrono` (with WASM-bindgen features) for client-side localized date/time computations.
  * `gloo` (event listeners) for scroll-sensitive navigation behaviors.
  * `web-sys` for programmatic browser DOM bindings.

---

## Component Layout & Structures

The application relies on a modular architecture to render the vintage layout:

```
src/
├── main.rs          # Entry point rendering the root component
├── app.rs           # Root Application component orchestrating layout panels
├── components/      # Functional interface components
│   ├── sidebar.rs   # Dispatch Menu navigation overlays
│   ├── warning.rs   # Desktop optimization responsive dialogs
│   ├── top.rs       # Header bar displaying localized datetime clocks
│   ├── head.rs      # Classic print masthead
│   ├── middle_head.rs
│   ├── news.rs      # Column wrapper for article modules
│   └── news_items/  # Content divisions containing specific project listings
├── layouts/         # Screen constraints and layout boundaries
└── utils/           # Time ticking clocks and helper logic
```

### Core Features

1. **Vintage Masthead & Layout Grid**
   Replicates early 20th-century print publications using double-bordered horizontal rules, drop-cap typography, and vintage parchment paper texture overlays.
   
2. **Interactive Dispatch Menu**
   A stateful slide-out navigation system. Built with event handlers that programmatically compute vertical viewport heights and scroll coordinates to seamlessly guide readers to distinct newspaper columns.

3. **Localized Real-Time Clock**
   Implements a custom Yew clock utility driven by client-side intervals that formats local timezone datetimes and updates every second.

4. **Desktop Layout Optimization HUD**
   Uses viewport queries to detect screen constraints, prompting users with a dismissible overlay recommending desktop viewports for ideal grid structures.

---

## Development & Build Pipeline

To compile and serve the project locally, install the standard Rust WebAssembly compiler pipeline.

### Prerequisites

1. **Rust Toolchain:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **WebAssembly Target:**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **Trunk Compiler:**
   ```bash
   cargo install --locked trunk
   ```

### Command Reference

* **Start Development Server:**
   ```bash
   trunk serve
   ```
   Runs a local development server at `http://localhost:8080` with hot-reloading.

* **Production Build Compilation:**
   ```bash
   trunk build --release
   ```
   Generates optimized WebAssembly binaries, CSS bundles, and static assets inside the `/dist` directory.

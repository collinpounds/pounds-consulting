# Pounds Consulting

> A consulting website built with Rust and WebAssembly. Because WordPress was too easy.

Yes, this is a brochure site compiled to the same technology that powers Figma, Google Earth, and AAA game engines. Is it overkill? Absolutely. Does it load faster than you can blink? Also yes.

**[poundsconsulting.net](https://poundsconsulting.net)**

## Why WebAssembly for a Consulting Site?

1. **It's blazingly fast** - Near-native performance on every device
2. **It proves we can** - If we'll over-engineer our own website, imagine what we'll do for your actual problems
3. **It's a conversation starter** - You're reading this, aren't you?

## Clone It, Make It Yours

This isn't just a website—it's a template. Fork it, gut it, rebuild it. The architecture is clean and the stack is modern.

### Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli

# Run it
dx serve
```

Open `http://localhost:8080` and watch a consulting website load at the speed of light.

### Build for Production

```bash
dx build --release
```

Output lands in `dist/`. Deploy anywhere static files are welcome.

## Project Structure

```
src/
├── main.rs              # Entry point + routing
├── components/          # Reusable UI pieces
│   ├── header.rs
│   ├── footer.rs
│   ├── cta_section.rs
│   └── service_card.rs
└── pages/               # The actual pages
    ├── home.rs
    ├── about.rs
    ├── services.rs
    └── contact.rs

assets/
└── main.css             # Glassmorphism + gold accents
```

## Design

- **Colors**: Black (`#0D0D0D`) + Gold (`#C9A227`)
- **Style**: Glassmorphism with backdrop blur
- **Fonts**: Montserrat, Open Sans, Inter
- **Responsive**: Mobile-first, looks good everywhere

## Customize

| Want to change... | Edit this |
|-------------------|-----------|
| Colors | `assets/main.css` (`:root` variables) |
| Content | `src/pages/*.rs` |
| Fonts | `index.html` (Google Fonts link) |

---

*Built with [Dioxus](https://dioxuslabs.com) + Rust. Deployed on GitHub Pages. Over-engineered with pride.*

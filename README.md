# Pounds Consulting

> A consulting website built with Rust and WebAssembly. Because WordPress was too easy.

Yes, this is a brochure site compiled to the same technology that powers Figma, Google Earth, and AAA game engines. Is it overkill? Absolutely. Does it load faster than you can blink? Also yes.

**[poundsconsulting.net](https://poundsconsulting.net)**

## Why WebAssembly for a Consulting Site?

1. **It's blazingly fast** - Near-native performance on every device
2. **It proves we can** - If we'll over-engineer our own website, imagine what we'll do for your actual problems
3. **It's a conversation starter** - You're reading this, aren't you?

## Features

### SEO Optimization
- **Sitemap** (`sitemap.xml`) - All pages indexed for search engines
- **Robots.txt** - Search engine crawling directives
- **LLMs.txt** - AI/LLM discovery file for emerging AI search
- **Schema.org Markup** - Structured data for rich search results (LocalBusiness, Organization)
- **SPA 404 Routing** - Custom 404.html for GitHub Pages client-side routing
- **Dynamic Meta Tags** - Per-page titles and descriptions

**Sitemap Structure:**
```
/                              # Home
/about                         # About
/services                      # Services
/portfolio                     # Portfolio listing
├── /portfolio/paytient
├── /portfolio/club-car-wash
├── /portfolio/old-hawthorne
├── /portfolio/gracie-humaita-columbia
├── /portfolio/att-indianapolis
├── /portfolio/apex-earthworks
├── /portfolio/missouri-jiu-jitsu
├── /portfolio/delaware-krav-maga
├── /portfolio/silo-wellness
└── /portfolio/toledo-aa
/articles                      # Articles listing
├── /articles/do-you-need-custom-website
├── /articles/red-flags-hiring-developer
├── /articles/questions-before-building-app
├── /articles/why-software-projects-fail
├── /articles/true-cost-free-website-builders
└── /articles/what-to-expect-working-with-us
/contact                       # Contact
```

### Portfolio Case Studies
- Individual case study pages at `/portfolio/:slug`
- Project logos
- Before/after comparison support
- Tech stack tags and scope of work
- Related projects section

### Content Management
- Full admin panel at `/admin` for managing articles and settings
- localStorage-based article storage (database-ready architecture)
- Feature toggles for discounts and navigation
- Import/export functionality for settings and content
- WYSIWYG article editor with preview

### Design System
- Dark theme with gold (#D4A017) accents
- Glassmorphism with backdrop blur effects
- Responsive mobile-first layout
- Montserrat, Open Sans, Inter fonts

## Clone It, Make It Yours

This isn't just a website, it's a template. Fork it, change it, rebuild it. The architecture is clean and the stack is modern.

### Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Dioxus CLI
cargo install dioxus-cli

# Run it (use dev.sh to include portfolio assets)
./dev.sh
# Or just: dx serve
```

Open `http://localhost:8080` and watch a consulting website load at the speed of light.

### Build for Production

```bash
dx build --release
```

Output lands in `target/dx/pounds-consulting/release/web/public/`. Deploy anywhere static files are welcome.

## Project Structure

```
src/
├── main.rs              # Entry point + routing
├── components/          # Reusable UI pieces
│   ├── header.rs        # Nav with active route detection
│   ├── footer.rs
│   ├── cta_section.rs
│   └── service_card.rs
├── content/             # Data layer
│   ├── types.rs         # Article, Portfolio, Settings structs
│   ├── storage.rs       # localStorage persistence
│   └── mod.rs
└── pages/               # The actual pages
    ├── home.rs
    ├── about.rs
    ├── services.rs
    ├── portfolio.rs
    ├── portfolio_detail.rs  # Individual case studies
    ├── articles.rs
    ├── article_detail.rs
    ├── contact.rs
    └── admin/           # Admin panel

assets/
├── main.css             # Gold accents and design secret sauce
└── portfolio/           # Project logos and screenshots

# SEO files (root level, copied to build)
├── sitemap.xml
├── robots.txt
├── llms.txt
└── 404.html
```

## Design

- **Colors**: Black (`#1A1A1A`) + Gold (`#D4A017`)
- **Style**: Glassmorphism with backdrop blur
- **Fonts**: Montserrat, Open Sans, Inter
- **Responsive**: Mobile-first, looks good everywhere

## Customize

| Want to change... | Edit this |
|-------------------|-----------|
| Colors | `assets/main.css` (`:root` variables) |
| Content | `src/pages/*.rs` |
| Portfolio data | `src/content/types.rs` (PortfolioData::default) |
| Fonts | `index.html` (Google Fonts link) |
| SEO | `sitemap.xml`, `robots.txt`, `llms.txt` |

## Deployment

GitHub Pages deployment is configured via `.github/workflows/deploy.yml`:
- Runs on push to `main`
- Checks formatting (`cargo fmt`)
- Runs Clippy lints
- Runs tests
- Builds with Dioxus CLI
- Copies SEO files and portfolio assets
- Deploys to GitHub Pages

---

*Built with [Dioxus](https://dioxuslabs.com) + Rust. Deployed on GitHub Pages. Over-engineered with pride.*

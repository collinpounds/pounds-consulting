# Pounds Consulting Website

A modern, professional website for Pounds Consulting built with Rust and Dioxus. Features a clean glassmorphism design with a black and gold color scheme.

## Prerequisites

1. **Install Rust**: https://rustup.rs/
2. **Install Dioxus CLI**:
   ```bash
   cargo binstall dioxus-cli
   # or
   cargo install dioxus-cli
   ```

## Running the Project

1. Navigate to the project directory:
   ```bash
   cd PoundsConsultingWebsite
   ```

2. Start the development server:
   ```bash
   dx serve
   ```

3. Open your browser to `http://localhost:8080`

## Building for Production

```bash
dx build --release
```

The built files will be in the `dist/` directory.

## Project Structure

```
PoundsConsultingWebsite/
├── Cargo.toml          # Rust dependencies
├── Dioxus.toml         # Dioxus configuration
├── index.html          # HTML template with SEO meta tags
├── assets/
│   └── main.css        # Styles with glassmorphism design
└── src/
    ├── main.rs         # App entry point with routing
    ├── components/     # Reusable UI components
    │   ├── mod.rs
    │   ├── header.rs
    │   ├── footer.rs
    │   ├── cta_section.rs
    │   └── service_card.rs
    └── pages/          # Page components
        ├── mod.rs
        ├── home.rs
        ├── about.rs
        ├── services.rs
        └── contact.rs
```

## Pages

- **Home** (`/`) - Hero, intro, services overview, why us section
- **About** (`/about`) - Story, values, experience, background
- **Services** (`/services`) - Detailed service offerings with pricing
- **Contact** (`/contact`) - Contact form, FAQ, scheduling

## Design Features

- **Glassmorphism cards** with backdrop blur effects
- **Responsive design** for mobile, tablet, and desktop
- **Gold accent color** (#C9A227) on black (#0D0D0D) base
- **Clean typography** using Montserrat, Open Sans, and Inter fonts
- **Smooth animations** and hover effects

## Customization

- Colors: Edit CSS variables in `assets/main.css` under `:root`
- Content: Modify the respective page files in `src/pages/`
- Fonts: Update the Google Fonts link in `index.html`

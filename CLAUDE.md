# Pounds Consulting Website Copy Guidelines

## Voice & Tone

### Who We Are
Pounds Consulting is a technical consulting firm run by Collin Pounds, based in Columbia, Missouri. We position ourselves as a **technical partner**, not a vendor or agency. The difference matters: we treat client businesses like their business and time is valuable and we are grateful they are using it with us, with direct communication and honest advice.

### Writing Style
- **Simple, plain English.** Avoid jargon. If a 5th grader can't understand it, rewrite it.
- **Direct and honest.** Don't soften bad news or oversell. Be truthful.
- **Conversational but professional.** Write like you're talking to a smart friend, not giving a corporate presentation.
- **Active voice.** "We build websites" not "Websites are built by us."

### What to Avoid
- **Em dashes (—).** Never use them. They're an AI telltale sign. Use commas, periods, parenthesis or rephrase.
- **Excessive superlatives.** Don't say "amazing," "incredible," "best-in-class." Let the work speak for itself.
- **Corporate buzzwords.** Avoid "synergy," "leverage," "holistic," "paradigm," etc.
- **Vague promises.** Don't say "we'll transform your business." Say what you'll actually do.
- **Filler words.** Cut "very," "really," "extremely" unless absolutely necessary.

## Key Messages

### Core Value Proposition
"Your Technical Partner" - We work alongside clients to build websites, fix technical problems, and make smart technology decisions. Direct communication. Honest advice. Work that lasts.

### What We Do (Simple Version)
1. **Website Development** - Fast, professional, custom websites that convert visitors
2. **Digital Marketing** - Email, SMS, scheduling, customer intake systems
3. **Technical Strategy** - Honest advice before you spend money on the wrong thing
4. **Business Solutions** - Connect software, automate tasks, fix technical problems

### Why Work With Us
1. **We keep it simple.** Every recommendation based on what works, not what sounds impressive.
2. **We show up.** Answer emails, meet deadlines, tell the truth even when uncomfortable.
3. **We've done this before.** Five years building software in healthcare, finance, and small business.
4. **No problem too big.** Network of brilliant specialists we can bring in for any challenge.

### The Network Advantage
Collin has a network of brilliant friends and collaborators across every specialty. This is a key differentiator:
- You get the personal attention of working with one person
- But you also get access to designers, developers, marketers, security specialists, data scientists, etc.
- Whatever the problem, someone in the network has solved it before
- No project is out of reach

**How to communicate this:**
- "We have a network of brilliant people we can call"
- "Need a designer? A security expert? We know the right person"
- "You get personal attention, backed by a team of experts when you need them"
- "Whatever the challenge, we know someone who's solved it"

**Avoid:**
- Making it sound like a faceless agency
- Overpromising on team size
- Vague claims like "we have resources"

## Content Structure

### Headlines
- Clear and direct
- State the benefit or address a pain point
- Examples:
  - "Your Technical Partner" (not "Innovative Digital Solutions")
  - "Work That Speaks for Itself" (not "Our Award-Winning Portfolio")
  - "Services Built Around Your Needs" (not "Comprehensive Service Offerings")

### Body Copy
- Lead with the reader's problem or goal
- Explain what we do in simple terms
- Show the benefit to them
- End with a clear next step

### Calls to Action
- Use action-oriented language
- Be specific about what happens next
- Examples:
  - "Let's Talk" / "Schedule a Call" / "Start a Conversation"
  - "Book a time" (casual, direct)
  - NOT "Learn More" or "Get Started" (too vague)

## Page-Specific Notes

### Home Page
- Hero should immediately communicate what we do and for whom
- Keep it short, visitors decide in seconds
- Lead with partnership angle, not services list

### About Page
- Use "we" language to leave room for future team growth
- Focus on relevant experience (Paytient, Club Car Wash, etc.)
- Emphasize values: simplicity, ownership mentality, built to last

### Services Page
- Lead each service with the client's problem
- Then explain what we do
- Include "Good Fit For" sections to help visitors self-qualify
- Pricing should be transparent ($100/hour, free discovery calls)

### Portfolio Page
- Let the work speak for itself
- Brief descriptions of what was done
- Include tech tags but keep them simple
- Link to live sites where available

### Articles
- Practical, helpful content that potential clients would find valuable
- Answer real questions people have before hiring someone
- Topics like:
  - "Do You Actually Need a Custom Website?"
  - "Red Flags When Hiring a Developer"
  - "5 Questions to Answer Before Building an App"
- No fluff, no filler, just useful information

## Technical Details

### Framework
- Built with Dioxus 0.7 (Rust-based WebAssembly framework)
- CSS uses custom properties (CSS variables) for theming
- Dark theme with gold (#D4A017) accent color
- Border radius set to 7px for a subtle, modern look

### Content Management
- Full admin panel at /admin for managing articles and settings
- Articles stored in localStorage (client-side, database-ready architecture)
- Settings control feature toggles (discounts, navigation visibility)
- Portfolio data defined in `src/content/types.rs` (PortfolioData::default)
- Import/export functionality for backup and migration
- Article editor with live preview, slug generation, and publish controls
- The admin system is architected to easily swap localStorage for a database backend

### SEO Implementation
The site includes comprehensive SEO optimization:

1. **Sitemap** (`sitemap.xml`)
   - All main pages indexed
   - Individual article pages with lastmod dates
   - Individual portfolio case study pages
   - Priority weighting (1.0 for home, 0.6 for articles/portfolio)

2. **Robots.txt** (`robots.txt`)
   - Allows all crawlers
   - Points to sitemap location

3. **LLMs.txt** (`llms.txt`)
   - AI/LLM discovery file for emerging AI-powered search
   - Describes services, contact info, and site structure

4. **Schema.org Markup** (in `index.html`)
   - LocalBusiness structured data
   - Organization data with contact info
   - Helps search engines understand the business

5. **SPA Routing** (`404.html`)
   - Custom 404 page that redirects to the SPA
   - Preserves client-side routing on GitHub Pages

### Portfolio System
- Data-driven architecture following the Article pattern
- `PortfolioProject` struct with: id, slug, title, project_type, description, long_description, external_url, before_url, logo, screenshot, video, tech_tags, scope
- Individual case study pages at `/portfolio/:slug`
- Logos stored in `assets/portfolio/`
- Deploy workflow copies assets to build output

### WASM Considerations
- Time functions use `js_sys::Date::now()` for WASM compatibility
- Conditional compilation with `#[cfg(target_arch = "wasm32")]`
- Avoid `std::time::SystemTime` in WASM context

### Development
- Use `./dev.sh` instead of `dx serve` to auto-sync portfolio assets
- Assets in `assets/portfolio/` need manual copying in dev mode
- Production build handles this via deploy.yml workflow

### Testing Before Commit
Always run these before committing to catch CI failures early:

```bash
# Install Clippy if not already installed
rustup component add clippy

# Run checks (same as CI)
cargo fmt --check       # Formatting
cargo clippy            # Lints
cargo test              # Tests
```

The GitHub Actions workflow runs all three, so running them locally prevents failed deployments.

## Example Rewrites

### Before (AI-sounding):
"We leverage cutting-edge technology to deliver innovative solutions that transform businesses and drive unprecedented growth, all while maintaining exceptional quality and client satisfaction."

### After (Pounds Consulting style):
"We build websites and solve technical problems. No jargon. No overcomplicated solutions. Just clean work that lasts."

### Before (em dash usage):
"Every project starts with understanding what you actually need, not what sounds impressive."

### After (comma instead):
"Every project starts with understanding what you actually need, not what sounds impressive."

### Before (em dash usage):
"We work alongside you to build websites, fix technical problems, and make smart technology decisions, whether you're starting something new or fixing something broken."

### After (split into sentences):
"We work alongside you to build websites, fix technical problems, and make smart technology decisions. Whether you're starting something new or fixing something broken, we can help."

## Quick Reference

**Do:**
- Use short sentences
- Write in plain English
- Be direct and honest
- Lead with client benefits
- Use commas and periods instead of em dashes

**Don't:**
- Use em dashes (—)
- Use corporate jargon
- Make vague promises
- Oversell or use superlatives
- Write long, complex sentences

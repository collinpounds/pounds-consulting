# Task Plan: Pounds Consulting Website

---

# v1.1 - Services Overhaul & Icon System [COMPLETED]

## Goal
Transform the services page into a compelling, professional presentation that makes visitors confident Collin can solve any technical problem, driving them to book a call.

## Changes Delivered
1. **Renamed**: Website Development → Web Development
2. **Added Services**:
   - AI Consulting (CS degree, AI/LLM/SLM expertise)
   - Mobile App Development
   - Product Management
3. **Design Improvements**:
   - Individual pages for each service (/services/:slug)
   - Persona cards replacing "Good Fit For" arrows
   - Accent color borders unique to each service
   - SVG icon system replacing all emojis (50+ icons)
   - Centered grid layouts
   - Animated cards with hover effects

## Phases
- [x] Phase 1: Explore current services implementation
- [x] Phase 2: Design new service structure and layout
- [x] Phase 3: Implement service changes and new sections
- [x] Phase 4: Update navigation and routing
- [x] Phase 5: Create SVG icon component system
- [x] Phase 6: Replace all emojis with icons
- [x] Phase 7: Review and refine styling

## Files Created/Modified
- `src/components/icon.rs` (new) - SVG icon component with 50+ icons
- `src/pages/service_detail.rs` (new) - Individual service pages
- `src/content/types.rs` - Service, Persona, ServicesData structs
- `src/content/storage.rs` - load_services() function
- `src/pages/services.rs` - Overview page with animated cards
- `src/pages/home.rs` - Updated with icon system
- `src/components/service_card.rs` - Updated with icons
- `src/main.rs` - ServiceDetail route and tests
- `assets/main.css` - ~500 lines of new CSS

## Status
**COMPLETED** - Committed as c88dada

---

# v1.2 - SEO & Content Strategy Overhaul [IN PROGRESS]

## Goal
Comprehensive content rewrite with authoritative backlinks, persuasive copy, and high-converting messaging for technical consulting services.

## Phases
- [x] Phase 1: Research current content and identify backlink opportunities
- [x] Phase 2: Compile authoritative sources list for approval
- [x] Phase 3: Rewrite content with SEO optimization and persuasion
- [x] Phase 4: Implement changes across all pages
- [ ] Phase 5: Update sitemap and meta tags (optional)

## Key Questions
1. What authoritative sources are relevant to each service area?
2. Which pages have the highest conversion potential?
3. What keywords should we target for each service?
4. How do we balance SEO with the CLAUDE.md voice guidelines?

## Backlink Strategy Categories

### Technology & Development
- Mozilla Developer Network (MDN) - https://developer.mozilla.org
- W3C Standards - https://w3.org
- Google Developers - https://developers.google.com
- web.dev - https://web.dev
- GitHub - https://github.com

### AI & Machine Learning
- Anthropic (Claude) - https://anthropic.com
- Hugging Face - https://huggingface.co
- Google AI - https://ai.google
- Stanford HAI - https://hai.stanford.edu
- Footprint - https://onefootprint.com

### Mobile Development
- Apple Developer - https://developer.apple.com
- Android Developers - https://developer.android.com
- React Native - https://reactnative.dev
- Flutter - https://flutter.dev

### Business & Strategy
- Ernst & Young (EY) - https://ey.com
- Gartner Research - https://gartner.com
- Forrester - https://forrester.com
- ThoughtWorks Radar - https://thoughtworks.com/radar
- a16z - https://a16z.com

### Product Management
- Tesla - https://tesla.com
- SpaceX - https://spacex.com
- The Boring Company - https://boringcompany.com

### Marketing & Analytics
- Google Analytics - https://analytics.google.com
- HubSpot - https://hubspot.com
- Mailchimp - https://mailchimp.com
- Klaviyo - https://klaviyo.com

### Integration & Automation
- Zapier - https://zapier.com
- Make (Integromat) - https://make.com
- Stripe - https://stripe.com
- Twilio - https://twilio.com

### Security & Compliance
- OWASP - https://owasp.org
- NIST - https://nist.gov

## Statistics for Persuasion
- "53% of mobile users abandon sites taking >3 seconds" (Google)
- "A 1-second delay decreases conversions by 7%" (Google/SOASTA)
- "AI adoption has doubled since 2017" (McKinsey)
- "Automation saves 20+ hours/week for small businesses" (Zapier)
- "Mobile accounts for 60%+ of web traffic" (Statista)

## Decisions Made
- Removed OpenAI as a source
- Added Footprint (onefootprint.com) as AI authority
- Added Ernst & Young (ey.com) as consulting credibility source
- Added Tesla, SpaceX, The Boring Company as Product Management references

## Errors Encountered
- (none)

## Changes Implemented
1. **All 7 Service Pages Rewritten** with authoritative backlinks:
   - AI Consulting: Anthropic, Hugging Face, Google AI, Stanford HAI, Footprint, EY
   - Web Development: MDN, W3C, Google Developers, web.dev, Stripe
   - Mobile App Development: Apple Developer, Android Developers, React Native, Flutter
   - Product Management: Tesla, SpaceX, The Boring Company, Gartner, Forrester
   - Digital Marketing: Mailchimp, Klaviyo, Twilio, HubSpot, Google Analytics
   - Technical Strategy: Gartner, Forrester, ThoughtWorks, EY, a16z
   - Business Solutions: Zapier, Make, Stripe, Twilio

2. **Home Page** updated with persuasive, benefit-focused copy
3. **About Page** updated with specific credibility (Paytient, Club Car Wash, etc.)
4. **CSS Fixed** - h3 headings in service detail pages now visually distinct with left border accent

## Status
**Phases 3-4 Complete** - Ready for review

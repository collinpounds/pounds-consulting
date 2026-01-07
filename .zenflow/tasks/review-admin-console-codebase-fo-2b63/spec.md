# Technical Specification: Admin Console Phase 1

## Technical Context

### Technology Stack
- **Framework**: Dioxus 0.7 (Rust-based WebAssembly framework)
- **Language**: Rust (2021 edition)
- **Build Target**: WebAssembly (wasm32)
- **Key Dependencies**:
  - `dioxus = "0.7"` with `web` and `router` features
  - `serde = "1.0"` with `derive` feature
  - `serde_json = "1.0"`
  - `web-sys = "0.3"` with `Window` and `Storage` features
  - `gloo-timers = "0.3"` with `futures` feature
  - `js-sys = "0.3"`

### Current Architecture Overview

```
src/
├── main.rs              # App entry, Route enum, Layout component
├── content/
│   ├── mod.rs           # Re-exports storage and types
│   ├── storage.rs       # localStorage abstraction layer
│   └── types.rs         # Data models (Article, PortfolioProject, SiteSettings)
├── components/          # Reusable UI components (Header, Footer, ServiceCard, etc.)
├── pages/
│   ├── mod.rs           # Page exports
│   ├── admin/
│   │   ├── mod.rs       # Admin page exports
│   │   ├── login.rs     # Password-based auth
│   │   ├── dashboard.rs # Stats overview, quick actions
│   │   ├── articles.rs  # Article list with trash support
│   │   ├── article_editor.rs  # Create/edit articles
│   │   └── settings.rs  # Site settings, feature toggles, discounts
│   └── [public pages]   # Home, About, Services, Portfolio, etc.
└── assets/
    └── main.css         # All styles including admin panel
```

### Current Storage Layer

**File**: `src/content/storage.rs`

| Function | Description |
|----------|-------------|
| `load_settings()` | Load `SiteSettings` from localStorage |
| `save_settings()` | Save `SiteSettings` to localStorage |
| `load_articles()` | Load `ArticlesData` from localStorage |
| `save_articles()` | Save `ArticlesData` to localStorage |
| `load_portfolio()` | **Returns hardcoded defaults only** |
| `is_authenticated()` | Check if `admin_auth` key exists |
| `set_authenticated()` | Set/clear auth token |
| `verify_password()` | Compare plain text password |
| `init_storage()` | Initialize with defaults if empty |
| `export_data()` / `import_data()` | Backup/restore (unused) |

**Storage Keys**:
- `site_settings` - SiteSettings JSON
- `site_articles` - ArticlesData JSON
- `admin_auth` - Simple "true" string for auth
- `articles_version` - Version tracking for migrations

### Current Data Models

**File**: `src/content/types.rs`

```rust
// Existing - working
pub struct Article {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub category: String,
    pub excerpt: String,
    pub content: String,
    pub status: ArticleStatus,  // Draft, Published, Trashed
}

// Existing - missing status field
pub struct PortfolioProject {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub project_type: String,
    pub description: String,
    pub long_description: String,
    pub external_url: String,
    pub before_url: Option<String>,
    pub logo: Option<String>,
    pub screenshot: Option<String>,
    pub video: Option<String>,
    pub tech_tags: Vec<String>,
    pub scope: Vec<String>,
    // MISSING: pub status: PortfolioStatus
}
```

### Current Admin UI Patterns

1. **Layout**: Three-column structure for articles
   - Fixed left sidebar (250px) with navigation
   - Secondary list sidebar (300px) for item selection
   - Main content area for preview/editing

2. **Authentication**: Simple password check, stores boolean in localStorage

3. **State Management**: Uses `use_signal()` for local component state

4. **CRUD Pattern** (Articles):
   - List view with All/Trash toggle
   - Select item to see preview
   - Edit navigates to separate editor page
   - Soft delete (move to trash) with confirmation modal
   - Permanent delete with confirmation modal

---

## Implementation Approach

### Phase 1 Delivery Structure

We will deliver Phase 1 in four incremental milestones:

1. **Milestone 1: Storage Layer Extension** (Foundation)
2. **Milestone 2: Portfolio CRUD in Admin** (Core Feature)
3. **Milestone 3: Article Editor Improvements** (UX Polish)
4. **Milestone 4: Bug Fixes and Architecture Cleanup** (Quality)

Each milestone produces a working, testable state.

---

## Source Code Structure Changes

### New Files to Create

```
src/pages/admin/
├── portfolio.rs           # Portfolio list page (mirrors articles.rs)
├── portfolio_editor.rs    # Portfolio create/edit page (mirrors article_editor.rs)
└── mod.rs                 # Update exports
```

### Files to Modify

| File | Changes |
|------|---------|
| `src/content/types.rs` | Add `PortfolioStatus` enum, add `status` field to `PortfolioProject`, add `PortfolioProject::new()` method |
| `src/content/storage.rs` | Add `PORTFOLIO_KEY`, `save_portfolio()`, update `load_portfolio()` to use localStorage |
| `src/main.rs` | Add three new routes: `/admin/portfolio`, `/admin/portfolio/new`, `/admin/portfolio/:id` |
| `src/pages/admin/mod.rs` | Export new portfolio modules |
| `src/pages/admin/dashboard.rs` | Add portfolio stats, update sidebar nav |
| `src/pages/admin/articles.rs` | Add sidebar nav link for portfolio |
| `src/pages/admin/article_editor.rs` | Add markdown preview, unsaved changes warning, slug validation |
| `src/pages/admin/settings.rs` | Add sidebar nav link for portfolio |
| `src/pages/admin/login.rs` | Add loading state during auth check |
| `assets/main.css` | Add portfolio admin styles (reuse article patterns) |

---

## Data Model / API / Interface Changes

### New Types

```rust
// Add to src/content/types.rs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PortfolioStatus {
    Draft,
    Published,
    Trashed,
}

impl Default for PortfolioStatus {
    fn default() -> Self {
        PortfolioStatus::Draft
    }
}
```

### Modified Types

```rust
// Modify PortfolioProject in src/content/types.rs

pub struct PortfolioProject {
    // ... existing fields ...
    #[serde(default)]
    pub status: PortfolioStatus,  // NEW FIELD
}

impl PortfolioProject {
    pub fn new() -> Self {
        let id = generate_id();
        Self {
            id: id.clone(),
            slug: String::new(),
            title: String::new(),
            project_type: String::new(),
            description: String::new(),
            long_description: String::new(),
            external_url: String::new(),
            before_url: None,
            logo: None,
            screenshot: None,
            video: None,
            tech_tags: Vec::new(),
            scope: Vec::new(),
            status: PortfolioStatus::Draft,
        }
    }

    pub fn generate_slug(title: &str) -> String {
        // Same logic as Article::generate_slug
    }
}
```

### New Storage Functions

```rust
// Add to src/content/storage.rs

const PORTFOLIO_KEY: &str = "site_portfolio";
const PORTFOLIO_VERSION_KEY: &str = "portfolio_version";
const CURRENT_PORTFOLIO_VERSION: &str = "v1";

/// Load portfolio from localStorage, migrating from defaults if needed
pub fn load_portfolio() -> PortfolioData {
    // Check version for migration
    let stored_version: Option<String> = get_from_storage(PORTFOLIO_VERSION_KEY);
    let needs_init = stored_version.is_none();

    if needs_init {
        // First time: migrate hardcoded data to localStorage
        let defaults = PortfolioData::default();
        save_portfolio(&defaults);
        set_to_storage(PORTFOLIO_VERSION_KEY, &CURRENT_PORTFOLIO_VERSION.to_string());
        return defaults;
    }

    get_from_storage(PORTFOLIO_KEY).unwrap_or_else(|| {
        let defaults = PortfolioData::default();
        save_portfolio(&defaults);
        defaults
    })
}

/// Save portfolio to localStorage
pub fn save_portfolio(portfolio: &PortfolioData) -> bool {
    set_to_storage(PORTFOLIO_KEY, portfolio)
}
```

### New Routes

```rust
// Modify Route enum in src/main.rs

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // ... existing routes ...

    // Admin Portfolio routes (add before end of admin section)
    #[route("/admin/portfolio")]
    AdminPortfolio {},
    #[route("/admin/portfolio/new")]
    AdminPortfolioNew {},
    #[route("/admin/portfolio/:id")]
    AdminPortfolioEdit { id: String },
}
```

---

## Delivery Phases

### Milestone 1: Storage Layer Extension

**Goal**: Enable portfolio data persistence without UI changes.

**Tasks**:
1. Add `PortfolioStatus` enum to `types.rs`
2. Add `status` field to `PortfolioProject` with `#[serde(default)]`
3. Add `PortfolioProject::new()` and `generate_slug()` methods
4. Add `PORTFOLIO_KEY` constant and `save_portfolio()`/`load_portfolio()` functions to `storage.rs`
5. Update `PortfolioData::default()` to include `status: PortfolioStatus::Published` for existing items
6. Update `init_storage()` to handle portfolio initialization

**Verification**:
- `cargo build --target wasm32-unknown-unknown` succeeds
- `cargo test` passes
- Existing portfolio public pages still work

### Milestone 2: Portfolio CRUD in Admin

**Goal**: Full portfolio management through admin panel.

**Tasks**:
1. Create `src/pages/admin/portfolio.rs` (list view with All/Trash toggle)
2. Create `src/pages/admin/portfolio_editor.rs` (create/edit form)
3. Add new routes to `src/main.rs`
4. Update `src/pages/admin/mod.rs` with new exports
5. Add Portfolio link to admin sidebar in all admin pages
6. Update dashboard to show portfolio stats (total, published, drafts)
7. Add portfolio-specific CSS styles to `main.css`

**Verification**:
- Can navigate to `/admin/portfolio`
- Can create new portfolio project
- Can edit existing portfolio project
- Can move project to trash
- Can restore from trash
- Can permanently delete
- Dashboard shows correct portfolio counts

### Milestone 3: Article Editor Improvements

**Goal**: Better article editing experience.

**Tasks**:
1. Add markdown preview pane (tab toggle: Edit/Preview)
2. Implement simple markdown-to-HTML rendering (headers, paragraphs, lists, bold, italic)
3. Add unsaved changes tracking with `use_signal`
4. Show warning modal when navigating away with unsaved changes
5. Add slug duplicate detection with visual warning
6. Auto-sanitize slug input (lowercase, hyphens, remove special chars)
7. Increase textarea default height and add auto-resize behavior

**Verification**:
- Preview tab shows rendered content
- Navigation blocked when changes pending
- Duplicate slug shows warning
- Invalid slug characters auto-corrected

### Milestone 4: Bug Fixes and Architecture Cleanup

**Goal**: Polish and prepare for future phases.

**Tasks**:
1. Add loading state during auth check (prevent content flash)
2. Ensure logout fully clears session and redirects
3. Add save confirmation toast to settings page
4. Validate discount percentage input (0-100)
5. Extract shared admin sidebar component (reduce duplication)
6. Add basic responsive styles for tablet view
7. Add localStorage quota warning
8. Ensure all existing tests pass
9. Add new tests for storage functions

**Verification**:
- No flash of admin content before redirect
- Settings shows save confirmation
- Admin usable on tablet (768px+)
- `cargo test` passes
- `dx build --release` succeeds without warnings

---

## Verification Approach

### Build Commands

```bash
# Development build
dx serve

# Release build (verifies no warnings)
dx build --release

# Run tests
cargo test

# Check WASM target
cargo build --target wasm32-unknown-unknown
```

### Manual QA Checklist

**Portfolio Management**:
- [ ] Create new portfolio project with all fields
- [ ] Edit existing project
- [ ] Move to trash
- [ ] View trash
- [ ] Restore from trash
- [ ] Permanently delete
- [ ] Verify public portfolio page shows only published items

**Article Editor**:
- [ ] Preview renders markdown correctly
- [ ] Unsaved changes warning appears
- [ ] Can dismiss warning and continue editing
- [ ] Duplicate slug shows warning
- [ ] Slug auto-sanitizes special characters

**General Admin**:
- [ ] Dashboard shows correct stats
- [ ] All sidebar links work
- [ ] Logout clears session
- [ ] Settings save successfully
- [ ] Works on tablet viewport (768px)

---

## Architecture Notes for Future Phases

### Preparing for Backend (Phase 2)

The current implementation uses synchronous localStorage functions. For Phase 2 backend integration:

1. **Storage trait abstraction**: Create a trait that both localStorage and API implementations can satisfy
2. **Async consideration**: Current functions are sync; API calls will need async. Consider `spawn_local` pattern
3. **ID generation**: Current timestamp-based IDs work for single-user; backend should generate UUIDs

### Preparing for User Roles (Phase 3)

1. **Auth state**: Move from simple boolean to user object with role
2. **Route guards**: Add permission checks per route
3. **UI conditionals**: Hide admin features based on role

### Preparing for Page Editor (Phase 4)

1. **Block-based content**: `PortfolioProject.long_description` and `Article.content` will become structured block arrays
2. **Component library**: Build reusable blocks (heading, paragraph, image, list)
3. **Drag-drop**: Consider libraries like `dioxus-sortable` when available

---

## Risk Mitigations

| Risk | Mitigation |
|------|------------|
| localStorage quota (~5MB) | Add usage indicator in settings; warn at 80% |
| Data loss on clear | Document export feature; auto-export on major changes |
| Markdown rendering in WASM | Use simple custom parser; avoid heavy crates |
| Breaking existing portfolio pages | Add status field with `#[serde(default)]` for backward compatibility |

---

## Dependencies and External Resources

- No new crate dependencies required for Phase 1
- May consider `pulldown-cmark` for markdown in future, but simple custom parser preferred for bundle size
- All styling uses existing CSS patterns from `main.css`

---

## Definition of Done

- [ ] All Milestone 1-4 tasks completed
- [ ] `cargo build --target wasm32-unknown-unknown` succeeds
- [ ] `dx build --release` succeeds without warnings
- [ ] `cargo test` passes
- [ ] Manual QA checklist completed
- [ ] No regressions in existing functionality

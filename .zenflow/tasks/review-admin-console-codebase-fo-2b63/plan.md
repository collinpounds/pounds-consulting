# Admin Console Phase 1 - Implementation Plan

## Configuration
- **Artifacts Path**: `.zenflow/tasks/review-admin-console-codebase-fo-2b63`

---

## Workflow Steps

### [x] Step: Requirements
<!-- chat-id: 37f77bc6-768d-486c-b8e6-ca24019fbc8a -->
Completed. See `requirements.md`.

### [x] Step: Technical Specification
<!-- chat-id: efc17f7f-9d04-4731-a44f-3c4c8336c30d -->
Completed. See `spec.md`.

### [x] Step: Planning
<!-- chat-id: ef12d724-63dd-4c26-bf9e-2bb64622655d -->
Completed. Implementation tasks defined below.

---

## Implementation Tasks

### Milestone 1: Storage Layer Extension

#### [ ] Task 1.1: Add PortfolioStatus enum to types.rs
**File**: `src/content/types.rs`

**Changes**:
- Add `PortfolioStatus` enum with variants: Draft, Published, Trashed
- Implement `Default` for `PortfolioStatus` (default to Draft)
- Add serde attributes for serialization

**Verification**:
- `cargo build --target wasm32-unknown-unknown` succeeds

---

#### [ ] Task 1.2: Update PortfolioProject with status field and helper methods
**File**: `src/content/types.rs`

**Changes**:
- Add `status: PortfolioStatus` field with `#[serde(default)]` for backward compatibility
- Add `PortfolioProject::new()` method that creates empty project with generated ID
- Add `PortfolioProject::generate_slug(title: &str)` method (mirror Article pattern)
- Update `PortfolioData::default()` to set status to `Published` for existing hardcoded items

**Verification**:
- `cargo build --target wasm32-unknown-unknown` succeeds
- Existing portfolio pages still render

---

#### [ ] Task 1.3: Add portfolio storage functions
**File**: `src/content/storage.rs`

**Changes**:
- Add `PORTFOLIO_KEY` and `PORTFOLIO_VERSION_KEY` constants
- Implement `save_portfolio(portfolio: &PortfolioData) -> bool`
- Update `load_portfolio()` to:
  - Check for existing localStorage data
  - Migrate from hardcoded defaults on first load
  - Save version marker
- Update `init_storage()` to initialize portfolio if needed

**Verification**:
- `cargo build --target wasm32-unknown-unknown` succeeds
- `cargo test` passes

---

### Milestone 2: Portfolio CRUD in Admin

#### [ ] Task 2.1: Add portfolio routes to main.rs
**File**: `src/main.rs`

**Changes**:
- Add to Route enum:
  - `#[route("/admin/portfolio")] AdminPortfolio {}`
  - `#[route("/admin/portfolio/new")] AdminPortfolioNew {}`
  - `#[route("/admin/portfolio/:id")] AdminPortfolioEdit { id: String }`

**Verification**:
- `cargo build --target wasm32-unknown-unknown` succeeds

---

#### [ ] Task 2.2: Create portfolio list page
**File**: `src/pages/admin/portfolio.rs` (new file)

**Changes**:
- Create `AdminPortfolio` component
- Implement list view with All/Trash toggle (mirror articles.rs pattern)
- Display: title, project_type, tech_tags, status
- Actions: Edit button, Delete button (soft delete with confirmation modal)
- Add auth check redirect
- Include admin sidebar with Portfolio link highlighted

**Verification**:
- Navigate to `/admin/portfolio` works
- List displays projects from localStorage
- Trash toggle works

---

#### [ ] Task 2.3: Create portfolio editor page
**File**: `src/pages/admin/portfolio_editor.rs` (new file)

**Changes**:
- Create `AdminPortfolioNew` and `AdminPortfolioEdit` components
- Form fields:
  - Title (text input, required)
  - Slug (text input, auto-generated from title)
  - Project Type (text input)
  - Short Description (textarea)
  - Long Description (large textarea, markdown)
  - External URL (text input)
  - Before URL (text input, optional)
  - Logo path (text input)
  - Screenshot path (text input)
  - Video path (text input, optional)
  - Tech Tags (text input, comma-separated)
  - Scope items (text input, comma-separated)
- Save/Publish/Cancel buttons
- Status management (Draft/Published)
- Include admin sidebar

**Verification**:
- Can create new portfolio project
- Can edit existing project
- Changes persist on refresh

---

#### [ ] Task 2.4: Update admin module exports
**File**: `src/pages/admin/mod.rs`

**Changes**:
- Add `pub mod portfolio;`
- Add `pub mod portfolio_editor;`
- Export new components

**Verification**:
- `cargo build --target wasm32-unknown-unknown` succeeds

---

#### [ ] Task 2.5: Update admin sidebar across all admin pages
**Files**: `src/pages/admin/dashboard.rs`, `src/pages/admin/articles.rs`, `src/pages/admin/settings.rs`

**Changes**:
- Add "Portfolio" link to sidebar navigation in each admin page
- Link to `/admin/portfolio`

**Verification**:
- Portfolio link visible and works from all admin pages

---

#### [ ] Task 2.6: Update dashboard with portfolio stats
**File**: `src/pages/admin/dashboard.rs`

**Changes**:
- Load portfolio data
- Add stats: Total portfolio items, Published, Drafts
- Show recent portfolio items in activity section (alongside articles)

**Verification**:
- Dashboard shows portfolio counts
- Counts are accurate

---

#### [ ] Task 2.7: Add portfolio admin CSS styles
**File**: `assets/main.css`

**Changes**:
- Add portfolio-specific styles (reuse article admin patterns)
- Form layout styles for portfolio editor
- Tag display styles

**Verification**:
- Portfolio pages render correctly with consistent styling

---

### Milestone 3: Article Editor Improvements

#### [ ] Task 3.1: Add markdown preview to article editor
**File**: `src/pages/admin/article_editor.rs`

**Changes**:
- Add Edit/Preview tab toggle
- Implement simple markdown-to-HTML renderer (headers, paragraphs, lists, bold, italic, links)
- Preview pane shows rendered content
- Tab state with `use_signal`

**Verification**:
- Preview tab shows rendered markdown
- Basic formatting works (headers, lists, bold, italic)

---

#### [ ] Task 3.2: Add unsaved changes warning
**File**: `src/pages/admin/article_editor.rs`

**Changes**:
- Track initial form state vs current state
- Set dirty flag when any field changes
- Show warning modal when navigating away with unsaved changes
- Options: Save, Discard, Cancel

**Verification**:
- Warning appears when leaving with unsaved changes
- No warning when no changes made
- Warning clears after saving

---

#### [ ] Task 3.3: Add slug validation and auto-sanitize
**File**: `src/pages/admin/article_editor.rs`

**Changes**:
- Auto-sanitize slug input: lowercase, replace spaces with hyphens, remove special chars
- Check for duplicate slugs against existing articles
- Show warning if slug already exists

**Verification**:
- Special characters stripped from slug
- Duplicate slug shows warning message

---

#### [ ] Task 3.4: Improve textarea UX
**File**: `src/pages/admin/article_editor.rs`

**Changes**:
- Increase default textarea height for content field
- Consider auto-resize based on content (CSS or JS)

**Verification**:
- Textarea is larger by default
- Better editing experience

---

### Milestone 4: Bug Fixes and Architecture Cleanup

#### [ ] Task 4.1: Fix auth flow issues
**Files**: `src/pages/admin/login.rs`, all admin pages

**Changes**:
- Add loading state during auth check (show spinner instead of content flash)
- Ensure logout fully clears session (`admin_auth` key)
- Redirect to login on all admin pages if not authenticated

**Verification**:
- No content flash on admin pages
- Logout works completely
- Unauthenticated access redirects to login

---

#### [ ] Task 4.2: Improve settings page
**File**: `src/pages/admin/settings.rs`

**Changes**:
- Add save confirmation toast/message
- Validate discount percentage (0-100)
- Show error for invalid inputs

**Verification**:
- Save shows confirmation
- Invalid discount rejected

---

#### [ ] Task 4.3: Add localStorage quota warning
**File**: `src/content/storage.rs` or `src/pages/admin/settings.rs`

**Changes**:
- Check localStorage usage
- Warn when approaching 80% of 5MB limit
- Display in settings page

**Verification**:
- Usage indicator shows in settings

---

#### [ ] Task 4.4: Extract shared admin sidebar component (optional refactor)
**File**: `src/components/admin_sidebar.rs` (new file, optional)

**Changes**:
- Create reusable AdminSidebar component
- Accept active page prop for highlighting
- Replace duplicated sidebar code in all admin pages

**Verification**:
- All admin pages use shared component
- Active state works correctly

---

#### [ ] Task 4.5: Add responsive styles for tablet
**File**: `assets/main.css`

**Changes**:
- Add media queries for 768px-1024px viewport
- Collapsible or narrower sidebar on tablet
- Ensure forms usable on tablet

**Verification**:
- Admin usable on iPad-sized viewport

---

#### [ ] Task 4.6: Run tests and verify build
**Commands**:
```bash
cargo test
cargo build --target wasm32-unknown-unknown
dx build --release
```

**Verification**:
- All tests pass
- Release build succeeds without warnings

---

## Verification Checklist

After all tasks complete:

- [ ] Portfolio CRUD works end-to-end
- [ ] Portfolio data persists in localStorage
- [ ] Article editor has markdown preview
- [ ] Unsaved changes warning works
- [ ] Dashboard shows portfolio stats
- [ ] Admin sidebar has portfolio link on all pages
- [ ] No auth flow issues
- [ ] Settings save with confirmation
- [ ] `cargo test` passes
- [ ] `dx build --release` succeeds
- [ ] No regressions in existing functionality

# Product Requirements Document: Admin Console Phase 1

## Overview

Improve the existing admin console to be more robust, add portfolio management capabilities, and prepare the architecture for future CRM features (user roles, backend integration, enhanced page editing).

**Scope:** Phase 1 focuses on improving what exists and adding portfolio CRUD. No backend changes, no user management yet.

## Goals

1. **Fix bugs and rough edges** in the current admin console
2. **Add portfolio management** to the admin panel (currently hardcoded in code)
3. **Improve the article editor** UX
4. **Clean up architecture** to support future backend integration
5. **Ensure the app builds and runs smoothly**

## Non-Goals (Future Phases)

- Backend/database integration (Phase 2)
- User authentication with email/password (Phase 2)
- Multi-user support and role-based access control (Phase 3)
- Block-based page editor (Phase 4)
- Editable public pages beyond articles/portfolio (Phase 4)

---

## Feature Requirements

### 1. Portfolio Management in Admin

**Current State:** Portfolio projects are hardcoded in `src/content/types.rs` via `PortfolioData::default()`. No way to add, edit, or delete portfolio items through the admin panel.

**Requirements:**

1.1. **Portfolio List Page** (`/admin/portfolio`)
- Display all portfolio projects in a list/grid view
- Show: title, project type, tech tags, status indicator
- Actions: Edit, Delete (soft delete with trash)
- Filter: All / Trash view (like articles)

1.2. **Portfolio Editor** (`/admin/portfolio/new` and `/admin/portfolio/:id`)
- Create new portfolio projects
- Edit existing projects
- Fields to edit:
  - Title (required)
  - Slug (auto-generated from title, editable)
  - Project Type (e.g., "Product Development", "Website", "Consulting")
  - Short Description (for cards/listings)
  - Long Description (full case study content, markdown support)
  - External URL (link to live project)
  - Before URL (optional, for before/after comparisons)
  - Logo path (text input for now, file upload in future phase)
  - Screenshot path (text input for now)
  - Video path (optional)
  - Tech Tags (comma-separated or tag input)
  - Scope items (list of deliverables)
- Save Draft / Publish functionality
- Status: Draft, Published, Trashed

1.3. **Portfolio Storage**
- Add `PORTFOLIO_KEY` to localStorage storage
- Create `save_portfolio()` and `load_portfolio()` functions that persist data
- Modify `load_portfolio()` to read from localStorage instead of returning hardcoded defaults
- Initialize with current hardcoded data on first load (migration)

1.4. **Navigation Updates**
- Add "Portfolio" link to admin sidebar navigation
- Update dashboard stats to include portfolio count

### 2. Article Editor Improvements

**Current State:** Basic textarea editor with metadata sidebar. Functional but minimal.

**Requirements:**

2.1. **Markdown Preview**
- Add a preview pane or toggle to see rendered markdown
- Split view option: editor on left, preview on right
- Or tab toggle: Edit / Preview

2.2. **Better Textarea Experience**
- Larger default height for content textarea
- Auto-resize textarea as content grows
- Preserve cursor position on re-renders

2.3. **Slug Validation**
- Warn if slug already exists (duplicate detection)
- Auto-sanitize slug (lowercase, hyphens only, no special chars)

2.4. **Unsaved Changes Warning**
- Track if form has unsaved changes
- Warn before navigating away if unsaved

### 3. Admin Console Bug Fixes and Polish

**Requirements:**

3.1. **Authentication Flow**
- Ensure logout fully clears session
- Redirect to login on any admin page if not authenticated
- Show loading state during auth check (prevent flash of content)

3.2. **Dashboard Improvements**
- Fix any stat calculation issues
- Add portfolio stats (total, published, drafts)
- Recent activity should show both articles and portfolio items

3.3. **Settings Page**
- Ensure all settings save correctly
- Add confirmation message on save
- Validate inputs (e.g., discount percentage 0-100)

3.4. **Responsive Design**
- Ensure admin pages work on tablet/mobile
- Collapsible sidebar on smaller screens

3.5. **Error Handling**
- Show user-friendly error messages on save failures
- Handle localStorage quota exceeded gracefully

### 4. Architecture Improvements

**Requirements:**

4.1. **Storage Layer Abstraction**
- Ensure all data access goes through `storage.rs`
- No direct localStorage calls in components
- Prepare function signatures for async (future backend)

4.2. **Type Safety**
- Add `PortfolioStatus` enum (Draft, Published, Trashed) matching `ArticleStatus`
- Ensure all IDs are generated consistently
- Add validation helpers for slugs, URLs

4.3. **Code Organization**
- Extract shared admin components (sidebar, header, stats cards)
- Create reusable form components (text input, textarea, select, toggle)
- Reduce code duplication between article and portfolio editors

4.4. **Testing Foundation**
- Ensure existing tests pass
- Add basic tests for new storage functions

---

## User Stories

### Portfolio Management

**As an admin, I want to:**
- View all my portfolio projects in one place
- Add a new portfolio project without editing code
- Edit an existing portfolio project's details
- Remove a portfolio project (move to trash)
- Recover a trashed portfolio project
- Permanently delete portfolio projects from trash

### Article Editing

**As an admin, I want to:**
- Preview how my article will look before publishing
- Be warned if I'm about to lose unsaved changes
- Know if my slug conflicts with an existing article

### General Admin

**As an admin, I want to:**
- See an overview of all my content (articles + portfolio) on the dashboard
- Have a smooth, bug-free experience managing content
- Access the admin panel on my tablet when away from my desk

---

## Technical Considerations

### Data Model Changes

**New/Modified in `types.rs`:**

```rust
// Add status to PortfolioProject
pub enum PortfolioStatus {
    Draft,
    Published,
    Trashed,
}

pub struct PortfolioProject {
    // ... existing fields ...
    pub status: PortfolioStatus,  // NEW
}
```

### Storage Changes

**New in `storage.rs`:**

```rust
const PORTFOLIO_KEY: &str = "site_portfolio";
const PORTFOLIO_VERSION_KEY: &str = "portfolio_version";

pub fn load_portfolio() -> PortfolioData {
    // Try localStorage first, fall back to defaults
    get_from_storage(PORTFOLIO_KEY).unwrap_or_else(|| {
        let defaults = PortfolioData::default();
        save_portfolio(&defaults);
        defaults
    })
}

pub fn save_portfolio(portfolio: &PortfolioData) -> bool {
    set_to_storage(PORTFOLIO_KEY, portfolio)
}
```

### New Routes

```rust
#[route("/admin/portfolio")]
AdminPortfolio {},
#[route("/admin/portfolio/new")]
AdminPortfolioNew {},
#[route("/admin/portfolio/:id")]
AdminPortfolioEdit { id: String },
```

### New Files

- `src/pages/admin/portfolio.rs` - Portfolio list page
- `src/pages/admin/portfolio_editor.rs` - Portfolio create/edit page
- `src/components/admin/` - Shared admin components (optional refactor)

---

## Success Criteria

1. **Portfolio CRUD works end-to-end:** Can create, read, update, delete portfolio projects through admin UI
2. **Data persists:** Portfolio changes survive page refresh (localStorage)
3. **Article editor improved:** Markdown preview works, unsaved changes warning works
4. **No regressions:** Existing functionality (articles, settings) still works
5. **App builds cleanly:** `cargo build` and `dx build --release` succeed without warnings
6. **Tests pass:** All existing and new tests pass

---

## Out of Scope Decisions

| Decision | Rationale |
|----------|-----------|
| No file upload for images | Requires backend; use path strings for now |
| No drag-drop reordering | Nice-to-have, adds complexity |
| No bulk operations | Single item operations sufficient for Phase 1 |
| No revision history | Requires more storage complexity |
| No scheduled publishing | Requires backend/cron |

---

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| localStorage size limits (~5MB) | Monitor usage, warn user if approaching limit |
| Data loss on browser clear | Document backup/export feature, encourage regular exports |
| Complex markdown rendering in WASM | Use simple markdown subset or client-side JS library |

---

## Definition of Done

- [ ] Portfolio admin pages implemented and functional
- [ ] Article editor has markdown preview
- [ ] Unsaved changes warning implemented
- [ ] Dashboard shows portfolio stats
- [ ] Admin sidebar includes portfolio link
- [ ] All existing tests pass
- [ ] New storage functions have basic tests
- [ ] App builds without errors or warnings
- [ ] Manual QA completed on key flows

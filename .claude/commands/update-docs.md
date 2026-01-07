# Update Documentation

Update all project documentation to reflect the current state of the codebase.

## Workflow

This skill runs through a documentation update workflow:

1. **Generate Code Summary** - Run `./generate-docs.sh` to compile all core Rust code into `docs/codebase-summary.md`
2. **Update README.md** - Review and update the README to reflect any new features, pages, or structural changes
3. **Update Website Content Doc** - Review and update `Pounds_Consulting_Website_Content.md` to match current site content

## Instructions

### Step 1: Generate Code Documentation

Run the generate-docs script:
```bash
./generate-docs.sh
```

This creates/updates `docs/codebase-summary.md` with all core Rust source files.

### Step 2: Update README.md

Review the current README and update it to reflect:
- Any new pages or routes added to the sitemap
- New features or components
- Changes to the project structure
- Updated build/deploy instructions if needed

Keep the README's tone - it's intentionally a bit cheeky about over-engineering a consulting site with WebAssembly.

### Step 3: Update Website Content Document

Review `Pounds_Consulting_Website_Content.md` and update:
- Sitemap structure if pages were added/removed
- Page content sections if copy changed
- Portfolio entries if new case studies were added
- Any new articles added
- Design system changes if applicable

This document serves as the source of truth for all website content.

## Output

After completing all steps, provide a summary of:
- What was generated/updated
- Any notable changes made
- Any sections that may need manual review

$ARGUMENTS

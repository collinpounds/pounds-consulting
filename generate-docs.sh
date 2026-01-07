#!/bin/bash
# Generate documentation by compiling all core Rust code into a single MD file for LLM review

OUTPUT_FILE="docs/codebase-summary.md"

# Create docs directory if it doesn't exist
mkdir -p docs

# Start the output file
cat > "$OUTPUT_FILE" << 'HEADER'
# Pounds Consulting Codebase Summary

> Auto-generated documentation for LLM review. Run `./generate-docs.sh` to regenerate.

This document contains all core Rust source code from the Pounds Consulting website.

---

## Table of Contents

HEADER

# Generate table of contents
echo "Building table of contents..."
{
    echo "- [Main Entry Point](#main-entry-point)"
    echo "- [Components](#components)"
    echo "- [Pages](#pages)"
    echo "- [Content/Data Layer](#contentdata-layer)"
    echo ""
    echo "---"
    echo ""
} >> "$OUTPUT_FILE"

# Function to add a file to the output
add_file() {
    local file=$1
    local title=$2
    local anchor=$3

    if [ -f "$file" ]; then
        echo "Adding $file..."
        {
            echo "## $title"
            echo ""
            echo "**File:** \`$file\`"
            echo ""
            echo '```rust'
            cat "$file"
            echo '```'
            echo ""
            echo "---"
            echo ""
        } >> "$OUTPUT_FILE"
    fi
}

# Main entry point
add_file "src/main.rs" "Main Entry Point" "main-entry-point"

# Components section
echo "# Components" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

add_file "src/components/mod.rs" "Components Module" "components-module"
add_file "src/components/header.rs" "Header Component" "header-component"
add_file "src/components/footer.rs" "Footer Component" "footer-component"
add_file "src/components/cta_section.rs" "CTA Section Component" "cta-section-component"
add_file "src/components/service_card.rs" "Service Card Component" "service-card-component"
add_file "src/components/theme_customizer.rs" "Theme Customizer Component" "theme-customizer-component"

# Pages section
echo "# Pages" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

add_file "src/pages/mod.rs" "Pages Module" "pages-module"
add_file "src/pages/home.rs" "Home Page" "home-page"
add_file "src/pages/about.rs" "About Page" "about-page"
add_file "src/pages/services.rs" "Services Page" "services-page"
add_file "src/pages/portfolio.rs" "Portfolio Page" "portfolio-page"
add_file "src/pages/portfolio_detail.rs" "Portfolio Detail Page" "portfolio-detail-page"
add_file "src/pages/articles.rs" "Articles Page" "articles-page"
add_file "src/pages/article_detail.rs" "Article Detail Page" "article-detail-page"
add_file "src/pages/contact.rs" "Contact Page" "contact-page"

# Content/Data layer section
echo "# Content/Data Layer" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

add_file "src/content/mod.rs" "Content Module" "content-module"
add_file "src/content/types.rs" "Content Types (Data Models)" "content-types"
add_file "src/content/storage.rs" "Storage Layer" "storage-layer"

# Add timestamp
{
    echo ""
    echo "---"
    echo ""
    echo "*Generated on $(date '+%Y-%m-%d %H:%M:%S')*"
} >> "$OUTPUT_FILE"

echo ""
echo "Documentation generated: $OUTPUT_FILE"
echo "Total lines: $(wc -l < "$OUTPUT_FILE")"

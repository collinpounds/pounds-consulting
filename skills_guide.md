# Claude Code Skills for Pounds Consulting

This document describes the available slash commands (skills) for this project.

---

## /write-article

**Purpose:** Generate article content for the Pounds Consulting website in Collin's voice.

**Usage:**
```
/write-article [topic]
```

**Examples:**
```
/write-article Why Your Business Needs a Mobile-Friendly Website
/write-article When to Hire a Freelancer vs an Agency
/write-article
```

If no topic is provided, Claude will ask what you'd like the article to be about.

**Output Format:**
The skill outputs a formatted article with:
- Title
- URL-friendly slug
- Category (Advice, Strategy, Technical, or About Us)
- Excerpt (1-2 sentence hook)
- Full article content in markdown

**Writing Style:**
- Direct and practical, no fluff
- Conversational but professional
- Short paragraphs (2-9 sentences)
- Uses contractions and simple words
- Avoids corporate jargon
- Leads with the reader's problem
- Ends with a soft CTA

**After Generation:**
Once the article is generated, you can add it to the site by:
1. Adding the article to `src/content/types.rs` in the `ArticlesData::default()` function
2. Adding the article URL to `sitemap.xml`
3. Optionally updating `Pounds_Consulting_Website_Content.md`

---

## Adding New Skills

To add a new skill:

1. Create a new `.md` file in `.claude/commands/`
2. Name it `your-skill-name.md` (this becomes `/your-skill-name`)
3. Include clear instructions and examples
4. Use `$ARGUMENTS` placeholder for user input
5. Document it in this file

**Skill Template:**
```markdown
# Skill Name

Description of what this skill does.

## Instructions

Detailed instructions for Claude...

$ARGUMENTS
```

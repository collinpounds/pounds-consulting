use serde::{Deserialize, Serialize};

/// Site-wide settings including branding and feature toggles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteSettings {
    pub brand: BrandSettings,
    pub features: FeatureToggles,
    pub pages: Vec<PageConfig>,
    pub admin_password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrandSettings {
    pub name: String,
    pub tagline: String,
    pub primary_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureToggles {
    pub portfolio: bool,
    pub services: bool,
    pub articles: bool,
    pub contact: bool,
    pub testimonials: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageConfig {
    pub id: String,
    pub label: String,
    pub path: String,
    pub enabled: bool,
    pub order: u32,
}

/// Article/blog post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub category: String,
    pub excerpt: String,
    pub content: String,
    pub status: ArticleStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArticleStatus {
    Draft,
    Published,
    Trashed,
}

/// Container for all articles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticlesData {
    pub articles: Vec<Article>,
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            brand: BrandSettings {
                name: "Pounds Consulting".to_string(),
                tagline: "Technical Solutions for Growing Businesses".to_string(),
                primary_color: "#C9A227".to_string(),
                accent_color: "#D4AF37".to_string(),
            },
            features: FeatureToggles {
                portfolio: true,
                services: true,
                articles: true,
                contact: true,
                testimonials: false,
            },
            pages: vec![
                PageConfig {
                    id: "home".to_string(),
                    label: "Home".to_string(),
                    path: "/".to_string(),
                    enabled: true,
                    order: 1,
                },
                PageConfig {
                    id: "about".to_string(),
                    label: "About".to_string(),
                    path: "/about".to_string(),
                    enabled: true,
                    order: 2,
                },
                PageConfig {
                    id: "services".to_string(),
                    label: "Services".to_string(),
                    path: "/services".to_string(),
                    enabled: true,
                    order: 3,
                },
                PageConfig {
                    id: "portfolio".to_string(),
                    label: "Portfolio".to_string(),
                    path: "/portfolio".to_string(),
                    enabled: true,
                    order: 4,
                },
                PageConfig {
                    id: "articles".to_string(),
                    label: "Articles".to_string(),
                    path: "/articles".to_string(),
                    enabled: true,
                    order: 5,
                },
                PageConfig {
                    id: "contact".to_string(),
                    label: "Contact".to_string(),
                    path: "/contact".to_string(),
                    enabled: true,
                    order: 6,
                },
            ],
            // Default password: "admin" - users should change this
            admin_password_hash: "admin".to_string(),
        }
    }
}

impl Default for ArticlesData {
    fn default() -> Self {
        Self {
            articles: vec![
                Article {
                    id: "do-you-need-custom-website".to_string(),
                    title: "Do You Actually Need a Custom Website?".to_string(),
                    slug: "do-you-need-custom-website".to_string(),
                    date: "2025-01-02".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "Before spending thousands on a custom site, here's how to figure out if you actually need one.".to_string(),
                    content: r#"A lot of businesses pay for custom websites when they don't need them. Here's how to figure out what's right for you.

## When a Template Works Fine

If your business fits a common pattern (local service business, restaurant, small retail shop) a template will probably work. Squarespace, Wix, and similar tools have gotten pretty good. You can have a professional-looking site up in a weekend for under $200/year.

Templates work when:
- Your site mostly shows information (hours, location, services, contact)
- You don't need to collect data or process payments in unusual ways
- You're okay looking similar to other businesses in your industry
- You can handle basic updates yourself

## When You Need Something Custom

Custom makes sense when:
- You need your website to DO something specific (booking systems, customer portals, calculators)
- You're integrating with other software your business uses
- Your business model doesn't fit standard templates
- Speed and performance are critical to your revenue
- You need to stand out in a crowded market

## The Middle Ground

There's a third option many people miss: start with a template, then add custom pieces. Use Squarespace for your main site, but build a custom tool for that one specific thing you need. This saves money and gets you moving faster.

## The Real Question

Don't ask "should I build custom?" Ask "what do I need this website to accomplish?" Start there, and the right answer usually becomes obvious.

If you're not sure, we're happy to talk through it. No sales pitch, just honest advice about what makes sense for your situation."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "red-flags-hiring-developer".to_string(),
                    title: "Red Flags When Hiring a Developer".to_string(),
                    slug: "red-flags-hiring-developer".to_string(),
                    date: "2025-01-01".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "How to spot problems before you've wasted time and money on the wrong hire.".to_string(),
                    content: r#"Hiring a developer or agency can feel like a gamble. Here are warning signs we've seen lead to bad outcomes.

## They Can't Explain Things Simply

Good developers can explain technical concepts in plain English. If someone hides behind jargon or makes you feel stupid for asking questions, that's a problem. You'll be working with this person, so communication matters.

## They Promise Everything Will Be Easy

Software is rarely easy. If someone says your project will be quick and cheap without asking many questions first, they either don't understand what you're asking for, or they're telling you what you want to hear.

## No Portfolio or References

Everyone has to start somewhere, but experienced developers should be able to show you past work. Ask for references. Actually call them. Ask what went wrong on the project (something always does) and how the developer handled it.

## They Want All the Money Upfront

Standard practice is milestone-based payments. Some money upfront is reasonable, but if they want 100% before starting, walk away. You lose all leverage if something goes wrong.

## They Don't Ask About Your Business

A developer who jumps straight to technical solutions without understanding your business goals will build the wrong thing. Good developers ask lots of questions first.

## Unusually Low Prices

If one bid is half the price of everyone else, something's wrong. Either they don't understand the scope, they're going to cut corners, or they'll hit you with change orders later.

## What to Look For Instead

- Clear communication
- Honest about challenges and tradeoffs
- Asks good questions about your goals
- Has relevant experience they can demonstrate
- Reasonable pricing with clear milestones
- Responsive during the sales process (it only gets worse after you sign)"#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "questions-before-building-app".to_string(),
                    title: "5 Questions to Answer Before Building an App".to_string(),
                    slug: "questions-before-building-app".to_string(),
                    date: "2024-12-28".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "Most app projects fail because people skip these questions. Don't be one of them.".to_string(),
                    content: r#"Before you spend money building an app, make sure you can answer these questions clearly.

## 1. What Problem Does This Solve?

Not "what would be cool to have" but what actual problem are real people experiencing that this app fixes? If you can't describe the problem in one sentence, you're not ready to build.

## 2. Who Exactly Will Use This?

"Everyone" is not an answer. Who specifically? How old are they? What do they do? Where will they find your app? The more specific you can be, the better your app will be.

## 3. How Will People Find It?

This is where most apps die. Building it is the easy part. Getting people to actually download and use it is hard. What's your plan? Be specific.

## 4. What's the Simplest Version That Solves the Problem?

Your first version should do one thing well. Not ten things. Not five things. One thing. You can add more later. Most apps fail because they try to do too much too soon.

## 5. How Will You Make Money?

Apps cost money to build and maintain. How does this pay for itself? Subscriptions? One-time purchase? Advertising? In-app purchases? If you don't have a clear answer, you'll run out of money before you succeed.

## Still Want to Build?

If you can answer all five questions clearly, you might be ready. If not, spend more time on these before writing any code. The cheapest time to change your mind is before you start building.

We're happy to help you think through these questions. Sometimes a 30-minute conversation saves months of wasted effort."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "why-software-projects-fail".to_string(),
                    title: "Why Most Software Projects Fail (And How to Avoid It)".to_string(),
                    slug: "why-software-projects-fail".to_string(),
                    date: "2024-12-20".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "After years of building software, we've seen the same mistakes over and over. Here's how to avoid them.".to_string(),
                    content: r#"Most software projects fail. Not because of bad code, but because of bad decisions made before any code was written.

## The Scope Keeps Growing

This is the number one killer. You start with a simple idea, then keep adding "just one more thing." Each feature seems small, but they add up. Before you know it, you're building something completely different from what you planned.

**How to avoid it:** Write down exactly what version 1 will do. Put it somewhere visible. Every time someone suggests a new feature, ask "Is this essential for launch, or can it wait for version 2?"

## Nobody Agrees on What "Done" Means

The project drags on forever because there's no clear finish line. Different people have different ideas of what success looks like.

**How to avoid it:** Before you start, define what "done" means in writing. What has to work? What can be imperfect? Get everyone to agree on this upfront.

## Building Before Validating

People spend months building something, then find out nobody wants it. Or they want something slightly different.

**How to avoid it:** Before building the full product, test your idea cheaply. Mock-ups, landing pages, manual versions of the process. Find out if people will actually pay for this before you invest heavily.

## Poor Communication

The developer builds what they understood, not what you meant. Weeks of work get thrown away because of a misunderstanding.

**How to avoid it:** Over-communicate. Check in frequently. Review work in progress, not just finished features. Ask "Can you show me what you have so far?" regularly.

## No One Is In Charge

When everyone is responsible, no one is. Decisions don't get made. Problems don't get solved.

**How to avoid it:** One person needs to be the decision-maker. They don't have to be right about everything, but someone has to be able to break ties and keep things moving.

## The Pattern

Notice something? Most of these problems are about communication and planning, not technology. The technical part is usually the easy part. Getting humans aligned is the hard part."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "true-cost-free-website-builders".to_string(),
                    title: "The True Cost of 'Free' Website Builders".to_string(),
                    slug: "true-cost-free-website-builders".to_string(),
                    date: "2024-12-15".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "Free sounds great until you add up what you're actually paying. Here's what those website builders really cost.".to_string(),
                    content: r#"Wix, Squarespace, and similar tools advertise low prices, but the actual cost is usually higher than it looks.

## The Monthly Fees Add Up

That $16/month plan is $192/year. Over 5 years, you've spent nearly $1,000, and you still don't own anything. Cancel your subscription and your site disappears.

## The Real Plans Cost More

The cheap plan usually has their branding on your site and missing features you'll need. Once you add a custom domain, remove ads, and get the features you actually need, you're often at $30-50/month.

## The Hidden Costs

- **Apps and plugins:** Many features require paid add-ons
- **Transaction fees:** Selling something? They take a cut
- **Storage and bandwidth:** Heavy use costs extra
- **Email:** Usually separate and extra
- **Support:** Good luck getting help on the cheap plans

## What You're Giving Up

### Portability
Your site lives on their platform. Want to move? You're starting over. That design, those pages, that SEO you built up, none of it transfers.

### Control
You can only do what their platform allows. Need something custom? Too bad. Their servers slow? Nothing you can do.

### Ownership
You're renting, not owning. They can change prices, features, or terms whenever they want.

## When It's Still Worth It

Despite all this, website builders make sense when:
- You need something up fast and cheap
- Your needs are simple and standard
- You're testing an idea before investing more
- You genuinely can't afford anything else right now

## The Alternative

A custom website costs more upfront but often less over time. You own it. You control it. You can move it. For a business that plans to be around for years, the math usually favors custom.

Run the numbers for your specific situation. Sometimes the "expensive" option is actually cheaper in the long run."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "what-to-expect-working-with-us".to_string(),
                    title: "What to Expect When Working With Us".to_string(),
                    slug: "what-to-expect-working-with-us".to_string(),
                    date: "2024-12-10".to_string(),
                    category: "About Us".to_string(),
                    excerpt: "Here's how we work with clients, what we expect from you, and what you can expect from us.".to_string(),
                    content: r#"Every consultant works differently. Here's how we do things so you know what you're getting into.

## How Projects Start

We start with a conversation, usually 30 minutes to an hour. No charge. We want to understand what you're trying to accomplish, what you've tried, and what's getting in the way.

After that, we'll tell you honestly whether we think we can help. Sometimes the answer is "you don't need us" or "someone else would be a better fit." We'd rather say that upfront than waste your time and money.

## How We Work

### Communication
We respond to emails within one business day. Usually faster. We believe in short, frequent check-ins rather than long silences followed by big reveals.

### Honesty
We tell you the truth, even when it's not what you want to hear. If your idea won't work, we'll say so. If something is taking longer than expected, you'll know right away.

### Simplicity
We look for the simplest solution that solves your problem. Not the coolest technology. Not the most impressive architecture. The simplest thing that works.

## What We Expect From You

### Availability
We need you to be reachable. Projects stall when we can't get answers to questions. You don't need to be available 24/7, but we need reasonable response times.

### Decisions
Someone needs to be able to make decisions. If every question requires a committee meeting, projects drag on forever.

### Honesty
If something isn't working for you, tell us. We can't fix problems we don't know about.

## Pricing

We work on a project basis with clear milestones and deliverables. You'll know the total cost before we start. We don't nickel-and-dime with change orders for small stuff.

For ongoing work, we offer monthly retainers. Fixed price, predictable costs.

## Ready to Talk?

If this sounds like a good fit, let's have a conversation. No commitment, no sales pressure. Just a chance to see if working together makes sense."#.to_string(),
                    status: ArticleStatus::Published,
                },
            ],
        }
    }
}

impl Article {
    pub fn new() -> Self {
        let id = generate_id();
        Self {
            id: id.clone(),
            title: String::new(),
            slug: String::new(),
            date: chrono_today(),
            category: "General".to_string(),
            excerpt: String::new(),
            content: String::new(),
            status: ArticleStatus::Draft,
        }
    }

    /// Generate a URL-friendly slug from the title
    pub fn generate_slug(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

/// Generate a simple unique ID
fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{:x}", duration.as_millis())
}

/// Get today's date in YYYY-MM-DD format
fn chrono_today() -> String {
    // Simple date - in a real app you'd use chrono crate
    "2024-01-01".to_string()
}

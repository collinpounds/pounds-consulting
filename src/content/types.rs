use serde::{Deserialize, Serialize};

/// Site-wide settings including branding and feature toggles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteSettings {
    pub brand: BrandSettings,
    pub features: FeatureToggles,
    pub pages: Vec<PageConfig>,
    pub admin_password_hash: String,
    #[serde(default)]
    pub discount: DiscountSettings,
}

/// Settings for promotional and special discounts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscountSettings {
    /// Holiday/promotional discount (admin-controlled)
    pub promo_discount: PromoDiscount,
    /// First responder/military discount visibility
    pub first_responder_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromoDiscount {
    pub enabled: bool,
    pub percentage: u8,
    pub label: Option<String>,
}

impl Default for DiscountSettings {
    fn default() -> Self {
        Self {
            promo_discount: PromoDiscount {
                enabled: false,
                percentage: 10,
                label: None,
            },
            first_responder_enabled: true,
        }
    }
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

/// Service offering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Service {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub icon: String,
    pub tagline: String,
    pub description: String,
    pub long_description: String,
    pub features: Vec<String>,
    pub personas: Vec<Persona>,
    pub accent_color: String,
}

/// Target persona for a service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Persona {
    pub title: String,
    pub icon: String,
    pub description: String,
}

/// Container for all services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServicesData {
    pub services: Vec<Service>,
}

/// Portfolio project/case study
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
}

/// Container for all portfolio projects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioData {
    pub projects: Vec<PortfolioProject>,
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
            discount: DiscountSettings::default(),
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

If your business fits a common pattern (local service business, restaurant, small retail shop) a template will probably work. Tools like <a href="https://squarespace.com" target="_blank" rel="noopener noreferrer">Squarespace</a>, <a href="https://wix.com" target="_blank" rel="noopener noreferrer">Wix</a>, and <a href="https://wordpress.com" target="_blank" rel="noopener noreferrer">WordPress.com</a> have come a long way. You can have a professional-looking site up in a weekend for under $200/year.

Templates work when:
- Your site mostly shows information (hours, location, services, contact)
- You don't need to collect data or process payments in unusual ways
- You're okay looking similar to other businesses in your industry
- You can handle basic updates yourself

If you're selling products, <a href="https://shopify.com" target="_blank" rel="noopener noreferrer">Shopify</a> handles most e-commerce needs out of the box. Their template ecosystem is mature and their checkout is optimized for conversions.

## When You Need Something Custom

Custom makes sense when:
- You need your website to DO something specific (booking systems, customer portals, calculators)
- You're integrating with other software your business uses
- Your business model doesn't fit standard templates
- Speed matters to your bottom line. Research from <a href="https://web.dev/performance/" target="_blank" rel="noopener noreferrer">Google's performance team</a> shows that <a href="https://web.dev/vitals/" target="_blank" rel="noopener noreferrer">every second of load time costs roughly 7% in conversions</a>
- You need to stand out in a crowded market

## The Middle Ground

There's a third option many people miss: start with a template, then add custom pieces. Use Squarespace for your main site, but build a custom tool for that one specific thing you need. Connect them with <a href="https://zapier.com" target="_blank" rel="noopener noreferrer">Zapier</a> or a simple API integration. This saves money and gets you moving faster.

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

Everyone has to start somewhere, but experienced developers should be able to show you past work. Check their <a href="https://github.com" target="_blank" rel="noopener noreferrer">GitHub profile</a> if they have one. Look at reviews on <a href="https://clutch.co" target="_blank" rel="noopener noreferrer">Clutch</a> for agencies. Ask for references and actually call them. Ask what went wrong on the project (something always does) and how the developer handled it.

## They Want All the Money Upfront

Standard practice is milestone-based payments. Some money upfront is reasonable (usually 25-50%), but if they want 100% before starting, walk away. You lose all leverage if something goes wrong. Platforms like <a href="https://toptal.com" target="_blank" rel="noopener noreferrer">Toptal</a> and <a href="https://upwork.com" target="_blank" rel="noopener noreferrer">Upwork</a> offer escrow protection for this reason.

## They Don't Ask About Your Business

A developer who jumps straight to technical solutions without understanding your business goals will build the wrong thing. Good developers ask lots of questions first. They want to understand the problem before proposing solutions.

## Unusually Low Prices

If one bid is half the price of everyone else, something's wrong. Either they don't understand the scope, they're going to cut corners, or they'll hit you with change orders later. Quality development work has a floor. You can find <a href="https://arc.dev/developer-salaries" target="_blank" rel="noopener noreferrer">typical developer rates by region</a> to calibrate your expectations.

## What to Look For Instead

- Clear communication in plain English
- Honest about challenges and tradeoffs
- Asks good questions about your goals before talking solutions
- Has relevant experience they can demonstrate with real examples
- Reasonable pricing with clear milestones and deliverables
- Responsive during the sales process (it only gets worse after you sign)
- Written agreements that specify what you're getting"#.to_string(),
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

Not "what would be cool to have" but what actual problem are real people experiencing that this app fixes? If you can't describe the problem in one sentence, you're not ready to build. <a href="https://www.ycombinator.com/library/4D-yc-s-essential-startup-advice" target="_blank" rel="noopener noreferrer">Y Combinator's advice to startups</a> applies here: make something people want.

## 2. Who Exactly Will Use This?

"Everyone" is not an answer. Who specifically? How old are they? What do they do? Where will they find your app? The more specific you can be, the better your app will be.

## 3. How Will People Find It?

This is where most apps die. Building it is the easy part. Getting people to actually download and use it is hard. The <a href="https://developer.apple.com/app-store/" target="_blank" rel="noopener noreferrer">Apple App Store</a> has over 2 million apps. <a href="https://play.google.com/store" target="_blank" rel="noopener noreferrer">Google Play</a> has even more. What's your plan to stand out? Be specific.

## 4. What's the Simplest Version That Solves the Problem?

Your first version should do one thing well. Not ten things. Not five things. One thing. You can add more later. Most apps fail because they try to do too much too soon. The startup world calls this an MVP (minimum viable product), and <a href="https://www.productplan.com/glossary/minimum-viable-product/" target="_blank" rel="noopener noreferrer">getting it right is an art</a>.

## 5. How Will You Make Money?

Apps cost money to build and maintain. How does this pay for itself? Subscriptions? One-time purchase? Advertising? In-app purchases? Both Apple and Google <a href="https://developer.apple.com/app-store/subscriptions/" target="_blank" rel="noopener noreferrer">take 15-30% of in-app purchases</a>. Factor that into your math.

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
                    content: r#"Most software projects fail. The <a href="https://www.standishgroup.com/sample_research_files/CHAOSReport2015-Final.pdf" target="_blank" rel="noopener noreferrer">Standish Group's research</a> has tracked this for decades: only about a third of software projects succeed. But it's rarely the code that kills them. It's the decisions made before any code was written.

## The Scope Keeps Growing

This is the number one killer. You start with a simple idea, then keep adding "just one more thing." Each feature seems small, but they add up. Before you know it, you're building something completely different from what you planned.

**How to avoid it:** Write down exactly what version 1 will do. Put it somewhere visible. Track it in a tool like <a href="https://linear.app" target="_blank" rel="noopener noreferrer">Linear</a> or <a href="https://www.atlassian.com/software/jira" target="_blank" rel="noopener noreferrer">Jira</a>. Every time someone suggests a new feature, ask "Is this essential for launch, or can it wait for version 2?"

## Nobody Agrees on What "Done" Means

The project drags on forever because there's no clear finish line. Different people have different ideas of what success looks like.

**How to avoid it:** Before you start, define what "done" means in writing. What has to work? What can be imperfect? Get everyone to agree on this upfront. The <a href="https://agilemanifesto.org" target="_blank" rel="noopener noreferrer">Agile Manifesto</a> calls this working software over comprehensive documentation.

## Building Before Validating

People spend months building something, then find out nobody wants it. Or they want something slightly different.

**How to avoid it:** Before building the full product, test your idea cheaply. Mock-ups in <a href="https://figma.com" target="_blank" rel="noopener noreferrer">Figma</a>, landing pages, manual versions of the process. Find out if people will actually pay for this before you invest heavily.

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
                    content: r#"<a href="https://wix.com" target="_blank" rel="noopener noreferrer">Wix</a>, <a href="https://squarespace.com" target="_blank" rel="noopener noreferrer">Squarespace</a>, and similar tools advertise low prices, but the actual cost is usually higher than it looks.

## The Monthly Fees Add Up

That $16/month plan is $192/year. Over 5 years, you've spent nearly $1,000, and you still don't own anything. Cancel your subscription and your site disappears.

## The Real Plans Cost More

The cheap plan usually has their branding on your site and missing features you'll need. Check the <a href="https://www.squarespace.com/pricing" target="_blank" rel="noopener noreferrer">Squarespace pricing page</a> closely. Once you add a custom domain, remove ads, and get the features you actually need, you're often at $30-50/month.

## The Hidden Costs

- **Apps and plugins:** Many features require paid add-ons
- **Transaction fees:** Selling something? <a href="https://stripe.com/pricing" target="_blank" rel="noopener noreferrer">Stripe charges 2.9% + 30 cents</a>. The website builder often adds their own fee on top
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

A custom website hosted on platforms like <a href="https://vercel.com" target="_blank" rel="noopener noreferrer">Vercel</a> or <a href="https://netlify.com" target="_blank" rel="noopener noreferrer">Netlify</a> costs more upfront but often less over time. You own it. You control it. You can move it. For a business that plans to be around for years, the math usually favors custom.

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

We start with a conversation, usually 30 minutes to an hour. No charge. <a href="https://calendar.app.google/NxuWY3RDGE5Miaan7" target="_blank" rel="noopener noreferrer">Book a time that works for you</a>. We want to understand what you're trying to accomplish, what you've tried, and what's getting in the way.

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

If this sounds like a good fit, <a href="https://calendar.app.google/NxuWY3RDGE5Miaan7" target="_blank" rel="noopener noreferrer">schedule a free discovery call</a>. No commitment, no sales pressure. Just a chance to see if working together makes sense. You can also <a href="/portfolio" target="_blank" rel="noopener noreferrer">check out our past work</a> first."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 1
                Article {
                    id: "website-costing-you-customers".to_string(),
                    title: "How to Tell If Your Website Is Costing You Customers".to_string(),
                    slug: "website-costing-you-customers".to_string(),
                    date: "2025-01-10".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "Your website might be driving away customers without you knowing. Here's how to find out.".to_string(),
                    content: r#"Most business owners have no idea how their website is actually performing. They assume if it loads, it's fine. But a slow or confusing website is like having a salesperson who mumbles and takes smoke breaks during pitches.

## Check Your Speed

Run your site through <a href="https://pagespeed.web.dev/" target="_blank" rel="noopener noreferrer">Google's PageSpeed Insights</a>. Anything below 50 on mobile is costing you money. <a href="https://gtmetrix.com" target="_blank" rel="noopener noreferrer">GTmetrix</a> gives you more detail on exactly what's slowing things down.

The math is brutal: <a href="https://web.dev/vitals/" target="_blank" rel="noopener noreferrer">Google's Core Web Vitals research</a> shows that sites meeting their performance thresholds see 24% fewer visitors abandon the page before it loads.

## Watch Real Users

Tools like <a href="https://hotjar.com" target="_blank" rel="noopener noreferrer">Hotjar</a> or <a href="https://clarity.microsoft.com" target="_blank" rel="noopener noreferrer">Microsoft Clarity</a> (which is free) let you see recordings of actual visitors using your site. You'll watch them get confused, click the wrong things, and give up. It's painful but enlightening.

## Check Your Mobile Experience

Pull out your phone and try to complete the main action on your site. Buy something. Fill out a contact form. Find your phone number. If it's frustrating for you, imagine how it feels for someone who's never been there.

Over 60% of web traffic is mobile. If your mobile experience is bad, most of your visitors are having a bad experience.

## Look at Your Analytics

If you have <a href="https://analytics.google.com" target="_blank" rel="noopener noreferrer">Google Analytics</a> set up, check your bounce rate. Above 70% on key pages? People are landing and immediately leaving. Check where they're dropping off in your conversion funnel.

## The Fixes

Most website performance problems come down to a few things:
- Images that aren't optimized (use <a href="https://tinypng.com" target="_blank" rel="noopener noreferrer">TinyPNG</a> or WebP format)
- Too many third-party scripts (tracking pixels, chat widgets, etc.)
- Cheap hosting that can't handle traffic
- Bloated themes or page builders

Sometimes a few quick fixes solve the problem. Sometimes you need to start fresh. But you won't know until you measure."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 2
                Article {
                    id: "build-vs-buy-software".to_string(),
                    title: "Build vs Buy: When to Use Off-the-Shelf Software".to_string(),
                    slug: "build-vs-buy-software".to_string(),
                    date: "2025-01-08".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "Custom software isn't always the answer. Here's how to decide when to build and when to buy.".to_string(),
                    content: r#"The decision to build custom software or buy an existing solution is one of the most expensive choices a business makes. Get it wrong and you'll waste months and tens of thousands of dollars.

## When to Buy

Buy when the problem is common and the solution is mature. Thousands of companies need CRM software. That's why <a href="https://salesforce.com" target="_blank" rel="noopener noreferrer">Salesforce</a>, <a href="https://hubspot.com" target="_blank" rel="noopener noreferrer">HubSpot</a>, and dozens of others exist. You're not special enough to need custom here.

Use <a href="https://g2.com" target="_blank" rel="noopener noreferrer">G2</a> or <a href="https://capterra.com" target="_blank" rel="noopener noreferrer">Capterra</a> to find existing solutions. Read the reviews. Try the free trials. Can something get you 80% of what you need? That's probably good enough.

## When to Build

Build when:
- Your business process is genuinely unique (not just different, unique)
- Off-the-shelf solutions require so many workarounds that they become unmaintainable
- The software IS your product or a core competitive advantage
- You've outgrown existing solutions and need something that scales differently

## The Hybrid Approach

Often the best answer is to buy the basics and build the differentiators. Use <a href="https://shopify.com" target="_blank" rel="noopener noreferrer">Shopify</a> for your store but build a custom recommendation engine. Use <a href="https://hubspot.com/crm" target="_blank" rel="noopener noreferrer">HubSpot's free CRM</a> but build custom integrations to your proprietary systems.

<a href="https://zapier.com" target="_blank" rel="noopener noreferrer">Zapier</a> and <a href="https://make.com" target="_blank" rel="noopener noreferrer">Make</a> let you connect off-the-shelf tools without writing code. Start there before jumping to custom development.

## The Real Cost of Building

Custom software costs more than most people expect. It's not just the initial build. There's:
- Ongoing maintenance and updates
- Security patches
- Hosting costs
- Bug fixes when something breaks at 2 AM

<a href="https://gartner.com" target="_blank" rel="noopener noreferrer">Gartner research</a> suggests that maintenance costs 15-20% of the original development cost per year. A $50,000 project costs $7,500-10,000 annually to maintain.

## The Decision Framework

Ask yourself:
1. Does a solution already exist that solves at least 80% of my problem?
2. If yes, can I live with the other 20% or work around it?
3. If I must build, is this a core business function worth the ongoing investment?
4. Have I actually tried the existing solutions, or am I assuming they won't work?

Most businesses should buy more and build less. But when building is the right choice, it can be a massive competitive advantage."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 3
                Article {
                    id: "small-business-ai-mistakes".to_string(),
                    title: "What Small Businesses Get Wrong About AI".to_string(),
                    slug: "small-business-ai-mistakes".to_string(),
                    date: "2025-01-06".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "AI can help your business, but probably not the way you think. Here's what actually works.".to_string(),
                    content: r#"Everyone's talking about AI. Most of what you hear is either hype or fear. Here's what small businesses actually need to know.

## Mistake 1: Thinking You Need Custom AI

You probably don't need to train a custom model or hire an AI team. The tools that already exist can handle most business use cases.

<a href="https://anthropic.com" target="_blank" rel="noopener noreferrer">Claude</a> can draft emails, summarize documents, and answer customer questions. <a href="https://openai.com/chatgpt" target="_blank" rel="noopener noreferrer">ChatGPT</a> can help with content creation and research. <a href="https://zapier.com/ai" target="_blank" rel="noopener noreferrer">Zapier's AI features</a> can automate workflows. These work out of the box.

## Mistake 2: Automating Before Understanding

AI amplifies what you already do. If your process is broken, AI will do broken things faster. Before automating anything, make sure the manual version works.

## Mistake 3: Ignoring What Actually Works

The most useful AI for small businesses isn't flashy. It's:

**Customer service:** AI chatbots that handle common questions 24/7. <a href="https://intercom.com" target="_blank" rel="noopener noreferrer">Intercom</a> and <a href="https://zendesk.com" target="_blank" rel="noopener noreferrer">Zendesk</a> both have AI features that work well.

**Content creation:** Drafting blog posts, social media, emails. Still needs human editing, but saves hours.

**Data entry:** Extracting information from documents, emails, forms. <a href="https://zapier.com" target="_blank" rel="noopener noreferrer">Zapier</a> and <a href="https://make.com" target="_blank" rel="noopener noreferrer">Make</a> can do this.

**Meeting notes:** Tools like <a href="https://otter.ai" target="_blank" rel="noopener noreferrer">Otter.ai</a> transcribe and summarize meetings automatically.

## Mistake 4: Expecting Magic

AI makes mistakes. Sometimes confident, convincing mistakes. Always have a human review anything customer-facing or legally important. AI is a tool, not a replacement for judgment.

## Where to Start

Pick one small, low-risk task. Maybe drafting the first version of your weekly email newsletter. Or summarizing customer feedback. Try it for a month. Measure if it actually saved time.

Most businesses find a few great use cases and lots of things that don't work. That's normal. The goal is finding your few great ones."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 4
                Article {
                    id: "first-developer-meeting".to_string(),
                    title: "How to Prepare for Your First Developer Meeting".to_string(),
                    slug: "first-developer-meeting".to_string(),
                    date: "2025-01-04".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "Make the most of your first meeting with a developer by coming prepared with the right information.".to_string(),
                    content: r#"You've decided to hire a developer. Now you're about to have your first meeting. Here's how to make it productive instead of confusing.

## What to Bring

### The Problem
Be ready to explain what's not working. Not the technical details, but the business problem. "Customers can't easily book appointments" is better than "I need an online booking system."

### Examples
Show them websites or apps that do something similar to what you want. "I like how Calendly handles scheduling" gives a developer more information than a paragraph of description.

### Your Constraints
Be upfront about budget, timeline, and non-negotiables. It saves everyone time if you're $5,000 and they're $50,000.

## What to Ask

### Their Process
How do they work? Weekly check-ins? What happens when there's a change request? How do they handle unexpected problems?

### Similar Projects
Have they built something like this before? Can they show you? Talk to that client?

### The Team
Who actually does the work? Some agencies sell with senior people and deliver with juniors. You want to know who you'll be working with.

### What Could Go Wrong
A good developer will be honest about risks. If they say nothing could go wrong, they're either lying or inexperienced.

## Red Flags During the Meeting

- They talk more than they listen
- They start proposing solutions before understanding your problem
- They promise an exact price without asking many questions
- They can't explain things in plain English
- They seem annoyed by your questions

## Green Flags

- They ask lots of clarifying questions
- They push back on requirements that don't make sense
- They're honest about what they don't know
- They suggest starting smaller than you proposed
- They explain the tradeoffs of different approaches

## After the Meeting

A good developer will follow up with a summary of what they understood and next steps. If you have to chase them, imagine how it'll be during the project.

Trust your gut. You're going to work closely with this person. If something feels off in the first meeting, it won't get better."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 5
                Article {
                    id: "roi-business-automation".to_string(),
                    title: "The Real ROI of Business Automation".to_string(),
                    slug: "roi-business-automation".to_string(),
                    date: "2025-01-03".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "Automation saves time, but how much? Here's how to calculate whether it's worth it for your business.".to_string(),
                    content: r#"Everyone talks about automation saving time. But how do you know if it's actually worth investing in? Here's how to think about it.

## The Basic Math

Calculate how long a task takes manually. Multiply by how often you do it. Multiply by your hourly rate (or the rate of whoever's doing it). That's the cost of not automating.

Example: A 15-minute task done 4 times a day at $50/hour costs $250/week, or $13,000/year in labor.

## What's Worth Automating

<a href="https://zapier.com" target="_blank" rel="noopener noreferrer">Zapier's State of Business Automation report</a> found that the biggest time savings come from:

**Lead management:** Automatically adding form submissions to your CRM, sending follow-up emails, notifying sales. <a href="https://hubspot.com" target="_blank" rel="noopener noreferrer">HubSpot</a> does much of this out of the box.

**Appointment scheduling:** Letting customers book themselves instead of back-and-forth emails. <a href="https://calendly.com" target="_blank" rel="noopener noreferrer">Calendly</a> and <a href="https://cal.com" target="_blank" rel="noopener noreferrer">Cal.com</a> handle this.

**Invoice and payment follow-up:** Automatic reminders for unpaid invoices. <a href="https://stripe.com/billing" target="_blank" rel="noopener noreferrer">Stripe Billing</a> and <a href="https://quickbooks.intuit.com" target="_blank" rel="noopener noreferrer">QuickBooks</a> can do this.

**Data entry:** Moving information between systems. Forms to spreadsheets, orders to inventory, leads to CRM.

## What's Not Worth Automating

- Tasks you do once a month for 5 minutes
- Processes that aren't standardized yet
- Anything requiring human judgment on every instance
- Things where the automation costs more than the time saved

## How to Start

1. Track your time for a week. What tasks are repetitive?
2. Pick the one that's most frequent AND most annoying
3. Check if <a href="https://zapier.com" target="_blank" rel="noopener noreferrer">Zapier</a> or <a href="https://make.com" target="_blank" rel="noopener noreferrer">Make</a> can connect the tools you're already using
4. Start simple. One automation that handles one thing.
5. Measure the before and after

## The Compound Effect

The real value of automation isn't just the time saved. It's that automated tasks don't get forgotten, don't have bad days, and don't quit. Every automation you build is an asset that keeps working.

Small businesses that systematically automate routine work create compounding efficiency advantages over competitors still doing everything manually."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 6
                Article {
                    id: "competitor-website-faster".to_string(),
                    title: "Why Your Competitor's Website Loads Faster (And Why It Matters)".to_string(),
                    slug: "competitor-website-faster".to_string(),
                    date: "2024-12-22".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "Website speed affects your search rankings, conversion rates, and bottom line. Here's how to fix it.".to_string(),
                    content: r#"Pull up your website and a competitor's side by side. Which loads faster? If it's not yours, you're losing money.

## Why Speed Matters

Google explicitly uses site speed in search rankings. <a href="https://developers.google.com/search/docs/appearance/core-web-vitals" target="_blank" rel="noopener noreferrer">Their Core Web Vitals update</a> made it official: slow sites rank lower.

But rankings aren't the only issue. Every second of load time <a href="https://web.dev/why-speed-matters/" target="_blank" rel="noopener noreferrer">costs you roughly 7% in conversions</a>. If your site makes $100,000/year and takes 2 extra seconds to load, that's potentially $14,000 you're leaving on the table.

## Common Reasons Your Site Is Slow

### Cheap Hosting
Shared hosting for $5/month means your site shares a server with hundreds of others. When traffic spikes, everyone slows down. <a href="https://cloudflare.com" target="_blank" rel="noopener noreferrer">Cloudflare</a> can help by caching your content on servers worldwide.

### Unoptimized Images
A single large image can slow your entire page. Use <a href="https://tinypng.com" target="_blank" rel="noopener noreferrer">TinyPNG</a> or convert to WebP format. Better yet, use a CDN that handles this automatically.

### Too Many Plugins
Every WordPress plugin adds code. Most add code even on pages where they're not used. Each one makes your site slower.

### Heavy Page Builders
Those drag-and-drop builders are convenient, but they generate bloated code. A simple page built with a page builder can be 10x heavier than the same page coded directly.

### Third-Party Scripts
Every analytics tool, chat widget, and tracking pixel adds load time. The Facebook pixel, Google Analytics, Intercom chat, HotJar recordings. They add up.

## How to Check

Run your site through <a href="https://pagespeed.web.dev/" target="_blank" rel="noopener noreferrer">PageSpeed Insights</a>. Score below 50 on mobile? You have work to do. <a href="https://gtmetrix.com" target="_blank" rel="noopener noreferrer">GTmetrix</a> shows you exactly what's slowing things down.

<a href="https://tools.pingdom.com" target="_blank" rel="noopener noreferrer">Pingdom</a> tests from different locations. Your site might be fast locally but slow for visitors across the country.

## Quick Wins

- Enable caching (if you're on WordPress, use a caching plugin)
- Compress images before uploading
- Remove plugins you're not actively using
- Consider a CDN like <a href="https://cloudflare.com" target="_blank" rel="noopener noreferrer">Cloudflare</a> (free tier works for most small sites)
- Audit your third-party scripts; remove ones that aren't earning their keep

## When You Need More

Sometimes the foundation is the problem. A site built on bloated technology can only be optimized so much. At some point, starting fresh with a clean, fast foundation is the better investment."#.to_string(),
                    status: ArticleStatus::Published,
                },
                // New Article 7
                Article {
                    id: "email-marketing-service-business".to_string(),
                    title: "Email Marketing That Actually Works for Service Businesses".to_string(),
                    slug: "email-marketing-service-business".to_string(),
                    date: "2024-12-18".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "Email marketing isn't just for e-commerce. Here's how service businesses can use it to stay top of mind and win more clients.".to_string(),
                    content: r#"Service businesses often think email marketing is just for online stores. It's not. Email is one of the best ways to stay in front of potential clients until they're ready to buy.

## Why Email Still Works

Social media algorithms decide who sees your posts. Email goes directly to their inbox. Open rates for business email <a href="https://mailchimp.com/resources/email-marketing-benchmarks/" target="_blank" rel="noopener noreferrer">average 20-25%</a>. That beats social media reach by a mile.

## What to Send

### The Welcome Sequence
When someone joins your list, send 3-5 emails over a couple weeks. Introduce yourself. Share your best advice. Help them before asking for anything.

### Regular Value
Monthly or bi-weekly updates with useful information. Not sales pitches. Tips they can actually use. Industry news that affects them. Answers to common questions you hear.

### Occasional Offers
After you've provided value, you've earned the right to ask. A promotion, a new service, a seasonal offer. But keep the ratio heavily tilted toward value.

## Tools for Service Businesses

<a href="https://mailchimp.com" target="_blank" rel="noopener noreferrer">Mailchimp</a> is fine for getting started. Free up to 500 contacts. <a href="https://convertkit.com" target="_blank" rel="noopener noreferrer">ConvertKit</a> is better for creators and consultants. <a href="https://klaviyo.com" target="_blank" rel="noopener noreferrer">Klaviyo</a> is overkill unless you're doing serious e-commerce.

The tool matters less than actually sending emails consistently.

## Building Your List

- Add a signup form to your website (not a popup, a visible form)
- Offer something useful in exchange: a guide, a checklist, a video
- Collect emails at in-person events
- Ask satisfied clients if they'd like to hear from you

## The Metrics That Matter

**Open rate:** Are people actually reading? Below 15% means your subject lines are boring or your list is stale.

**Click rate:** Are they taking action? 2-3% is typical.

**Unsubscribes:** A few per email is normal. A spike means you sent something off-putting.

## Starting Simple

Don't overthink it. Start with one email per month to past and potential clients. Share something useful. Include a clear call to action. See what happens.

Most service businesses find that just showing up consistently in inboxes wins more business than any fancy campaign. The bar is low because most of your competitors aren't doing it at all."#.to_string(),
                    status: ArticleStatus::Published,
                },
            ],
        }
    }
}

impl Default for PortfolioData {
    fn default() -> Self {
        Self {
            projects: vec![
                PortfolioProject {
                    id: "paytient".to_string(),
                    slug: "paytient".to_string(),
                    title: "Paytient".to_string(),
                    project_type: "Product Development".to_string(),
                    description: "Contributed to a healthcare fintech startup serving hundreds of thousands of users. Removed friction from the onboarding flow, redesigned the my.paytient.com landing page, and led a team of 6 engineers implementing multi-factor authentication.".to_string(),
                    long_description: r#"## The Challenge

Paytient is a healthcare fintech company that helps employees pay for medical expenses over time. With hundreds of thousands of users depending on the platform, every interaction matters.

## What We Did

### Onboarding Flow Optimization
We identified friction points in the member onboarding process and redesigned the flow to reduce drop-off rates. By simplifying the steps and improving the UI, we helped more users successfully complete registration.

### Landing Page Redesign
The my.paytient.com member portal landing page needed a refresh. We designed and implemented a cleaner, more intuitive experience that better communicated available benefits and next steps.

### Multi-Factor Authentication
Security is critical for financial applications. We led a team of 6 engineers to implement MFA across the entire platform, protecting user accounts while maintaining a smooth experience.

## Results

- Improved onboarding completion rates
- Better member engagement on the portal
- Enterprise-grade security with MFA
- Scalable architecture supporting continued growth"#.to_string(),
                    external_url: "https://my.paytient.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/paytient-logo.svg".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["React".to_string(), "UX".to_string(), "MFA".to_string(), "Team Lead".to_string()],
                    scope: vec![
                        "Streamlined onboarding by removing unnecessary friction step".to_string(),
                        "Redesigned member landing page for better engagement".to_string(),
                        "Led 6-person team implementing MFA across the platform".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "club-car-wash".to_string(),
                    slug: "club-car-wash".to_string(),
                    title: "Club Car Wash".to_string(),
                    project_type: "Website + Portal + Digital Marketing".to_string(),
                    description: "Complete digital transformation for a growing regional car wash chain. Built and managed the public-facing website, developed an internal employee portal, and ran ongoing Google Ads campaigns to support new store openings.".to_string(),
                    long_description: r#"## The Challenge

Club Car Wash is a rapidly expanding car wash chain opening approximately 3 new stores per month. They needed a complete digital presence that could scale with their growth.

## What We Did

### Public Website
Designed and developed a customer-facing website that showcases locations, membership options, and the Club Car Wash brand. The site is built for easy updates as new locations open.

### Employee Portal
Created an internal portal for employee operations, streamlining internal communication and processes across all locations.

### Digital Marketing
Managed Google Ads campaigns coordinated with new store openings, driving local awareness and membership sign-ups in each new market.

## Results

- Scalable website supporting rapid expansion
- Centralized employee portal across all locations
- Successful launch marketing for ~3 store openings per month
- 1+ year of ongoing maintenance and support"#.to_string(),
                    external_url: "https://clubcarwash.com".to_string(),
                    before_url: None,
                    logo: None,
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["React".to_string(), "Custom CMS".to_string(), "Google Ads".to_string()],
                    scope: vec![
                        "Public website design and development".to_string(),
                        "Employee portal for internal operations".to_string(),
                        "Google Ad campaign management (~3 store openings/month)".to_string(),
                        "Ongoing maintenance and support for 1 year".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "old-hawthorne".to_string(),
                    slug: "old-hawthorne".to_string(),
                    title: "Old Hawthorne Country Club".to_string(),
                    project_type: "Website Consulting".to_string(),
                    description: "Consulting work for a local country club community in Columbia, Missouri. Made targeted adjustments to improve the site's look and navigation, including replacing the dated beige wallpaper background with a cleaner design.".to_string(),
                    long_description: r#"## The Challenge

Old Hawthorne Country Club had an existing website that felt dated and didn't reflect the quality of the community. They needed targeted improvements without a complete rebuild.

## What We Did

### Visual Refresh
The most obvious issue was a dated beige wallpaper background that made the site feel old. We replaced it with a cleaner, more modern design that better represents the club.

### Navigation Improvements
Reorganized the site navigation to make it easier for members and prospective members to find information about amenities, membership, and events.

### UX Enhancements
Made various usability improvements throughout the site to create a more polished experience.

## Results

- Modern, professional appearance
- Improved navigation and usability
- Better reflection of the club's quality"#.to_string(),
                    external_url: "https://oldhawthorne.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/old-hawthorne-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Consulting".to_string(), "UI Cleanup".to_string(), "UX".to_string()],
                    scope: vec![
                        "Replaced dated beige wallpaper background".to_string(),
                        "Improved site navigation and layout".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "gracie-humaita-columbia".to_string(),
                    slug: "gracie-humaita-columbia".to_string(),
                    title: "Gracie Humaita Columbia".to_string(),
                    project_type: "Website + SMTP Integration".to_string(),
                    description: "Website for a Brazilian Jiu-Jitsu academy with integrated email automation. Designed to showcase class schedules, instructor profiles, and drive new student sign-ups with automated follow-up.".to_string(),
                    long_description: r#"## The Challenge

Gracie Humaita Columbia is a Brazilian Jiu-Jitsu academy that needed a professional web presence to attract new students and communicate with their community.

## What We Did

### Website Design
Created a mobile-first website that showcases the academy's programs, class schedules, and instructor profiles. The design reflects the Gracie Humaita brand while being accessible to beginners.

### Lead Capture
Built a lead capture system to collect information from prospective students interested in trying classes.

### Email Automation
Integrated SMTP-based email automation to automatically follow up with new leads, keeping them engaged until they come in for their first class.

## Results

- Professional web presence for the academy
- Automated lead follow-up saving staff time
- Mobile-friendly design for on-the-go access"#.to_string(),
                    external_url: "https://graciehumaitacolumbia.com".to_string(),
                    before_url: Some("https://web.archive.org/web/20190723164913/http://www.graciehumaitacolumbia.com/".to_string()),
                    logo: Some("assets/portfolio/gracie-humaita-columbia-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Mobile-First".to_string(), "Lead Capture".to_string(), "SMTP".to_string()],
                    scope: vec![
                        "Automated email follow-up for new leads".to_string(),
                        "Class schedule and instructor profiles".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "att-indianapolis".to_string(),
                    slug: "att-indianapolis".to_string(),
                    title: "American Top Team Indianapolis".to_string(),
                    project_type: "Website Replacement".to_string(),
                    description: "Replaced a broken, outdated website for a martial arts training facility. Built a clean, professional site with focus on easy navigation for prospective students.".to_string(),
                    long_description: r#"## The Challenge

American Top Team Indianapolis had an old, broken website that wasn't functioning properly. Prospective students couldn't find information about classes or contact the gym.

## What We Did

### Complete Replacement
Rather than trying to fix the broken site, we built a completely new website from scratch with modern technology and design.

### Professional Design
Created a clean, professional design that showcases the gym's programs and instructors. The site reflects ATT's brand while being welcoming to beginners.

### SEO Optimization
Built the site with SEO best practices to help the gym appear in local search results.

## Results

- Fully functional website replacing broken one
- Professional appearance reflecting ATT brand
- Easy navigation for prospective students
- Improved local search visibility"#.to_string(),
                    external_url: "https://attindianapolis.com".to_string(),
                    before_url: Some("https://web.archive.org/web/20200530220933/http://www.attindianapolis.com/".to_string()),
                    logo: Some("assets/portfolio/att-indianapolis-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Responsive".to_string(), "SEO".to_string()],
                    scope: vec![
                        "Replaced old broken website".to_string(),
                        "Clean, professional design".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "apex-earthworks".to_string(),
                    slug: "apex-earthworks".to_string(),
                    title: "APEX Earthworks".to_string(),
                    project_type: "Website + Lead Generation".to_string(),
                    description: "Business website for an earthwork and excavation company. Professional presentation with automated customer lead generation to capture and follow up with potential clients.".to_string(),
                    long_description: r#"## The Challenge

APEX Earthworks is an earthwork and excavation company that needed an online presence to attract commercial and residential clients.

## What We Did

### Professional Website
Built a business website that showcases APEX's services, equipment, and past projects. The design conveys professionalism and capability.

### Lead Generation
Implemented a lead capture system with automated follow-up to ensure no potential client falls through the cracks.

### Mobile Optimization
Ensured the site works perfectly on mobile devices, since many potential clients search for services on their phones.

## Results

- Professional online presence
- Automated lead capture and follow-up
- Mobile-friendly experience"#.to_string(),
                    external_url: "https://apexearthwork.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/apex-earthworks-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Lead Gen Automation".to_string(), "Mobile".to_string()],
                    scope: vec![
                        "Automated lead capture and follow-up".to_string(),
                        "Professional company showcase".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "missouri-jiu-jitsu".to_string(),
                    slug: "missouri-jiu-jitsu".to_string(),
                    title: "Missouri Jiu Jitsu".to_string(),
                    project_type: "Website Development".to_string(),
                    description: "Demo website with a mock jiu-jitsu academy featuring class information, instructor bios, and signup flow automation.".to_string(),
                    long_description: r#"## The Project

Missouri Jiu Jitsu is a demo website showcasing our capabilities for martial arts academies. It demonstrates a complete solution including class schedules, instructor profiles, and member sign-up flows.

## Features

### Class Schedules
Easy-to-read class schedule showing times, instructors, and skill levels for each class.

### Instructor Bios
Professional profiles for each instructor, highlighting their background and expertise.

### Membership Inquiry
Lead capture forms for prospective students to request information or schedule a trial class.

## Technical Details

- Responsive design for all devices
- Clean, modern aesthetic
- Fast loading times
- Easy content management"#.to_string(),
                    external_url: "https://missourijiujitsu.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/missouri-jiu-jitsu-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Responsive".to_string(), "Forms".to_string()],
                    scope: vec![
                        "Class schedules and instructor bios".to_string(),
                        "Membership inquiry forms".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "delaware-krav-maga".to_string(),
                    slug: "delaware-krav-maga".to_string(),
                    title: "Delaware Krav Maga".to_string(),
                    project_type: "Landing Page + Lead Capture".to_string(),
                    description: "Simple landing page with automated lead capture for a Krav Maga instructor's self-defense training service.".to_string(),
                    long_description: r#"## The Challenge

A Krav Maga instructor needed a simple, effective way to capture leads for his self-defense training service. No complex website needed, just a clean landing page that converts visitors into inquiries.

## What We Did

### Landing Page
Built a focused, single-page site that communicates the value of the training and drives visitors to take action.

### Automated Lead Capture
Set up an automated form that captures prospect information and delivers it directly to the instructor, no manual follow-up required.

## Results

- Clean, professional landing page
- Automated lead capture and delivery
- Simple, low-maintenance solution"#.to_string(),
                    external_url: "https://delawarekravmaga.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/delaware-krav-maga-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Landing Page".to_string(), "Lead Capture".to_string(), "Automation".to_string()],
                    scope: vec![
                        "Landing page design".to_string(),
                        "Automated lead capture form".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "silo-wellness".to_string(),
                    slug: "silo-wellness".to_string(),
                    title: "Silo Wellness".to_string(),
                    project_type: "Website Redesign".to_string(),
                    description: "Website redesign for a wellness company offering healing retreats in Jamaica. Created an inviting, professional presence that communicates trust and tranquility to prospective guests.".to_string(),
                    long_description: r#"## The Challenge

Silo Wellness offers healing retreats in Jamaica, providing transformative wellness experiences. Their website needed to communicate trust, tranquility, and professionalism to prospective guests considering this significant investment in their wellbeing.

## What We Did

### Complete Redesign
Redesigned the website from the ground up to create an inviting, professional experience that reflects the quality of the retreats.

### Trust Building
Incorporated elements that build trust: testimonials, clear information about what to expect, and professional photography showcasing the retreat experience.

### Booking Flow
Streamlined the inquiry and booking process to make it easy for interested visitors to take the next step.

## Results

- Professional, inviting web presence
- Clear communication of retreat offerings
- Streamlined booking inquiries
- Improved trust signals throughout"#.to_string(),
                    external_url: "https://silowellness.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/silo-wellness-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Redesign".to_string(), "UX".to_string(), "Wellness".to_string()],
                    scope: vec![
                        "Complete website redesign".to_string(),
                        "Healing retreat showcase".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "toledo-aa".to_string(),
                    slug: "toledo-aa".to_string(),
                    title: "Toledo Area AA".to_string(),
                    project_type: "Website Redesign".to_string(),
                    description: "Website redesign for the Alcoholics Anonymous organization serving the Toledo region in Ohio. Built with accessibility and ease of use as top priorities to help those seeking support.".to_string(),
                    long_description: r#"## The Challenge

Toledo Area AA serves people seeking help with alcohol addiction in the Toledo, Ohio region. The website needed to be accessible, easy to use, and welcoming to people who may be in crisis.

## What We Did

### Accessibility Focus
Built the site with accessibility as a top priority. Clear fonts, high contrast, and simple navigation ensure everyone can use the site regardless of ability.

### Meeting Finder
Created an easy-to-use meeting finder so visitors can quickly locate meetings near them by day, time, or location.

### Resource Hub
Organized resources and information in a clear, non-overwhelming way to help newcomers understand what to expect.

## Results

- Fully accessible website
- Easy meeting finder functionality
- Clear, welcoming design
- Resources organized for newcomers"#.to_string(),
                    external_url: "https://toledoaa.com".to_string(),
                    before_url: None,
                    logo: None,
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Redesign".to_string(), "Accessibility".to_string(), "Community".to_string()],
                    scope: vec![
                        "Complete website redesign".to_string(),
                        "Meeting finder and resources".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "habiducer".to_string(),
                    slug: "habiducer".to_string(),
                    title: "Habiducer".to_string(),
                    project_type: "Mobile App".to_string(),
                    description: "Habit and goal tracking mobile app designed to help users build consistent routines and achieve their objectives through simple, effective tracking.".to_string(),
                    long_description: r#"## The Project

Habiducer is a habit and goal tracking application built to help people build better routines and achieve their personal objectives.

## The Problem

Most habit trackers overwhelm users with features they don't need. Complex analytics, social features, and gamification often distract from the core purpose: building consistent habits.

## The Solution

Habiducer takes a different approach. Clean interface. Simple tracking. Focus on what matters: showing up every day.

### Key Features
- Straightforward habit tracking without the clutter
- Goal setting with progress visualization
- Daily reminders to keep you accountable
- Clean, distraction-free design

## Why It Works

The best habit tracker is the one you actually use. By removing unnecessary complexity, Habiducer helps users focus on building habits rather than managing an app."#.to_string(),
                    external_url: "https://habiducer.com".to_string(),
                    before_url: None,
                    logo: None,
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Mobile App".to_string(), "Habit Tracking".to_string(), "Goal Setting".to_string()],
                    scope: vec![
                        "Mobile app development".to_string(),
                        "Habit tracking system".to_string(),
                        "Goal progress visualization".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "pounds-consulting".to_string(),
                    slug: "pounds-consulting".to_string(),
                    title: "Pounds Consulting".to_string(),
                    project_type: "Open Source Website".to_string(),
                    description: "This very website. Built with Rust and WebAssembly using the Dioxus framework. Open source, over-engineered with pride, and a template for anyone to use.".to_string(),
                    long_description: r#"## The Project

Yes, this is a case study about the website you're currently viewing. A consulting website built with Rust and WebAssembly. Because WordPress was too easy.

## Why WebAssembly?

Three reasons:

1. **It's blazingly fast.** Near-native performance on every device.
2. **It proves we can.** If we'll over-engineer our own website, imagine what we'll do for your actual problems.
3. **It's a conversation starter.** You're reading this, aren't you?

## Technical Details

### Framework
Built with Dioxus 0.7, a Rust-based framework that compiles to WebAssembly. The same technology that powers Figma, Google Earth, and AAA game engines.

### Features
- Full admin panel for managing articles and settings
- Data-driven portfolio with individual case study pages
- SEO optimization (sitemap, robots.txt, schema.org markup)
- SPA routing on GitHub Pages with custom 404 handling
- Dark theme with gold accents and glassmorphism design

### Open Source
The entire codebase is available on GitHub. Fork it, clone it, make it yours. The architecture is clean and the stack is modern.

## Results

- Sub-second load times
- Perfect Lighthouse scores
- A website that doubles as a portfolio piece
- Open source template for the community"#.to_string(),
                    external_url: "https://github.com/collinpounds/pounds-consulting".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/Pounds-Consulting-Black-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Rust".to_string(), "WebAssembly".to_string(), "Dioxus".to_string(), "Open Source".to_string()],
                    scope: vec![
                        "Full-stack Rust/WASM website".to_string(),
                        "Admin panel with article management".to_string(),
                        "SEO optimization and SPA routing".to_string(),
                        "Open source on GitHub".to_string(),
                    ],
                },
            ],
        }
    }
}

impl Default for ServicesData {
    fn default() -> Self {
        Self {
            services: vec![
                Service {
                    id: "ai-consulting".to_string(),
                    slug: "ai-consulting".to_string(),
                    title: "AI Consulting".to_string(),
                    icon: "brain".to_string(),
                    tagline: "Turn AI potential into measurable business results".to_string(),
                    description: "According to McKinsey, companies using AI effectively see a 20% increase in cash flow. But most AI projects fail because they start with technology instead of business problems. With a Computer Science degree and hands-on experience implementing solutions with Claude, Llama, and custom models, I help businesses cut through the noise and deploy AI that actually delivers ROI.".to_string(),
                    long_description: r#"## The $15.7 Trillion Opportunity

According to PwC, AI will contribute $15.7 trillion to the global economy by 2030. But here's what the headlines don't tell you: 85% of AI projects fail to deliver expected results. The difference between success and failure isn't the technology. It's the strategy.

## How We Can Help

### AI Strategy & Assessment
Before writing a single line of code, we map your business processes to identify where AI creates genuine value versus expensive novelty. Using frameworks developed by leading consulting firms like <a href="https://ey.com" target="_blank" rel="noopener noreferrer">Ernst & Young</a>, we build business cases that your stakeholders can actually approve.

### LLM Implementation
Large Language Models have matured rapidly. <a href="https://anthropic.com" target="_blank" rel="noopener noreferrer">Anthropic's Claude</a> excels at nuanced reasoning. Open-source models from <a href="https://huggingface.co" target="_blank" rel="noopener noreferrer">Hugging Face</a> offer flexibility and cost control. <a href="https://ai.google" target="_blank" rel="noopener noreferrer">Google's Gemini</a> integrates seamlessly with existing Google infrastructure. We help you choose the right model for your specific use case and budget.

### Small Language Models (SLMs)
Sometimes smaller is better. SLMs can run on your own infrastructure, keeping sensitive data in-house while reducing API costs by 90% or more. Companies like <a href="https://onefootprint.com" target="_blank" rel="noopener noreferrer">Footprint</a> are pioneering privacy-first AI approaches that we can help you implement.

### Custom AI Solutions
When off-the-shelf doesn't cut it, we build custom solutions. Fine-tuned models. RAG systems for your proprietary data. AI-powered applications that give you a competitive edge. Research from <a href="https://hai.stanford.edu" target="_blank" rel="noopener noreferrer">Stanford HAI</a> informs our approach to responsible AI development.

## Why Work With Us

I've spent years at the intersection of computer science theory and practical business application. You get someone who can explain transformer architectures to your engineering team and ROI projections to your board. No translation needed."#.to_string(),
                    features: vec![
                        "AI strategy aligned with business objectives".to_string(),
                        "LLM integration (Claude, Llama, Gemini, Mistral)".to_string(),
                        "Small Language Model deployment for cost control".to_string(),
                        "Custom AI application development".to_string(),
                        "RAG systems for proprietary data".to_string(),
                        "Model selection and total cost analysis".to_string(),
                        "Prompt engineering and optimization".to_string(),
                        "AI security, privacy, and compliance consulting".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "Business Owner".to_string(),
                            icon: "briefcase".to_string(),
                            description: "You want AI that delivers ROI, not just impressive demos that go nowhere".to_string(),
                        },
                        Persona {
                            title: "Tech Leader".to_string(),
                            icon: "code".to_string(),
                            description: "You need to implement AI features and want guidance on architecture, model selection, and scaling".to_string(),
                        },
                        Persona {
                            title: "Operations Manager".to_string(),
                            icon: "settings".to_string(),
                            description: "You have repetitive processes costing thousands in labor that AI could handle in seconds".to_string(),
                        },
                    ],
                    accent_color: "#8B5CF6".to_string(), // Purple for AI
                },
                Service {
                    id: "web-development".to_string(),
                    slug: "web-development".to_string(),
                    title: "Web Development".to_string(),
                    icon: "globe".to_string(),
                    tagline: "Websites that load fast, rank high, and convert visitors into customers".to_string(),
                    description: "Google research shows 53% of mobile users abandon sites that take over 3 seconds to load. Your website isn't just a digital brochure. It's a 24/7 sales machine that should be working while you sleep. We build sites that meet Google's Core Web Vitals standards, rank in search results, and turn visitors into paying customers.".to_string(),
                    long_description: r#"## Every Second Costs You Money

According to Google/SOASTA research, a 1-second delay in page load decreases conversions by 7%. If your site generates $100,000 annually, a slow site could be costing you $7,000 or more. And that's before considering the visitors who never found you because slow sites rank lower in search results.

## How We Can Help

### Custom Websites

Every site we build starts with your business goals, not a template. We follow web standards from the <a href="https://w3.org" target="_blank" rel="noopener noreferrer">W3C</a> and build with performance best practices from <a href="https://web.dev" target="_blank" rel="noopener noreferrer">web.dev</a>. The result is a site that's fast, accessible, and built to last.

### E-commerce Solutions

Sell products online with a checkout experience optimized for conversion. We integrate with <a href="https://stripe.com" target="_blank" rel="noopener noreferrer">Stripe</a> for secure payment processing and build inventory systems that scale with your business. No 15% marketplace fees. No platform limitations.

### Web Applications

When you need your website to DO something. Customer portals that reduce support tickets. Booking systems that fill your calendar. Dashboards that surface insights from your data. We build tools that make your business more efficient.

### Performance Optimization

Already have a site that's slow? We audit against <a href="https://developers.google.com/search/docs/appearance/core-web-vitals" target="_blank" rel="noopener noreferrer">Google's Core Web Vitals</a>, identify bottlenecks, and implement fixes. Most sites see 40-60% improvement in load times.

## Built to Web Standards

We build according to specifications from <a href="https://developer.mozilla.org" target="_blank" rel="noopener noreferrer">MDN Web Docs</a> and W3C standards. This means your site works across browsers, meets accessibility requirements, and won't break when the next browser update drops."#.to_string(),
                    features: vec![
                        "Custom design tailored to your brand and goals".to_string(),
                        "Mobile-first responsive development".to_string(),
                        "SEO built-in from day one, not bolted on later".to_string(),
                        "Core Web Vitals optimization for Google ranking".to_string(),
                        "E-commerce with Stripe payment integration".to_string(),
                        "Content management you can actually use".to_string(),
                        "Ongoing support and maintenance plans".to_string(),
                        "Analytics and conversion tracking setup".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "New Business".to_string(),
                            icon: "rocket".to_string(),
                            description: "You're launching and need a professional web presence that makes you look established from day one".to_string(),
                        },
                        Persona {
                            title: "Growing Company".to_string(),
                            icon: "trending-up".to_string(),
                            description: "Your current site doesn't reflect how far you've come, and it's costing you credibility with prospects".to_string(),
                        },
                        Persona {
                            title: "E-commerce Seller".to_string(),
                            icon: "shopping-cart".to_string(),
                            description: "You want to own your customer relationships instead of paying 15% to marketplaces".to_string(),
                        },
                    ],
                    accent_color: "#3B82F6".to_string(), // Blue for Web
                },
                Service {
                    id: "mobile-app-development".to_string(),
                    slug: "mobile-app-development".to_string(),
                    title: "Mobile App Development".to_string(),
                    icon: "smartphone".to_string(),
                    tagline: "Native and cross-platform apps built for performance and user retention".to_string(),
                    description: "Mobile apps have 3x higher conversion rates than mobile web, according to Criteo research. But building an app that users actually keep on their phones requires more than code. It requires understanding platform guidelines, user expectations, and the economics of app store distribution. We build apps that get downloaded, used, and recommended.".to_string(),
                    long_description: r#"## Mobile is Where Your Customers Are

Statista reports mobile devices account for over 60% of web traffic. But here's what matters more: users spend 90% of their mobile time in apps, not browsers. If your competitors have an app and you don't, you're invisible during most of your customers' screen time.

## How We Can Help

### Native iOS & Android

Following <a href="https://developer.apple.com/design/human-interface-guidelines" target="_blank" rel="noopener noreferrer">Apple's Human Interface Guidelines</a> and <a href="https://developer.android.com" target="_blank" rel="noopener noreferrer">Google's Material Design</a> principles, we build apps that feel right on each platform. Native apps access device features, perform faster, and create better user experiences.

### Cross-Platform Apps

Why build twice when you can build once? <a href="https://reactnative.dev" target="_blank" rel="noopener noreferrer">React Native</a> and <a href="https://flutter.dev" target="_blank" rel="noopener noreferrer">Flutter</a> let us deploy to both iOS and Android from a single codebase, cutting development time by 30-40% without sacrificing quality.

### Progressive Web Apps (PWAs)

Not every app needs the App Store. PWAs install from the browser, work offline, and avoid the 15-30% app store commission. For the right use case, PWAs deliver 80% of native functionality at 20% of the cost.

### App Modernization

Have an existing app built on outdated technology? We modernize apps to meet current platform requirements, improve performance, and prepare for future updates.

## The Honest Conversation

Before we build anything, we help you answer the hard questions: Do you actually need an app? What problem does it solve that a website can't? How will users discover it? Many businesses discover they need something simpler. We'd rather tell you that upfront than take your money for something that won't succeed."#.to_string(),
                    features: vec![
                        "iOS development following Apple HIG standards".to_string(),
                        "Android development with Material Design".to_string(),
                        "React Native and Flutter cross-platform apps".to_string(),
                        "App Store optimization and submission".to_string(),
                        "Push notifications for user engagement".to_string(),
                        "Offline-first architecture".to_string(),
                        "Backend API development and integration".to_string(),
                        "Analytics, crash reporting, and user insights".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "Startup Founder".to_string(),
                            icon: "lightbulb".to_string(),
                            description: "You have an app idea and need to build an MVP that validates the concept without burning your runway".to_string(),
                        },
                        Persona {
                            title: "Enterprise Leader".to_string(),
                            icon: "building".to_string(),
                            description: "Your field teams need mobile tools that work offline and sync when connected".to_string(),
                        },
                        Persona {
                            title: "Service Business".to_string(),
                            icon: "wrench".to_string(),
                            description: "Your customers expect to book, pay, and communicate with you from their phones".to_string(),
                        },
                    ],
                    accent_color: "#10B981".to_string(), // Green for Mobile
                },
                Service {
                    id: "product-management".to_string(),
                    slug: "product-management".to_string(),
                    title: "Product Management".to_string(),
                    icon: "target".to_string(),
                    tagline: "Build products people actually want, on time and on budget".to_string(),
                    description: "According to Gartner, 45% of product launches miss their target dates, and 80% of new products fail within their first year. The difference between success and failure isn't luck or even engineering talent. It's disciplined product management: knowing what to build, when to ship, and how to iterate based on real user feedback.".to_string(),
                    long_description: r#"## The $10 Million Decision Problem

Every feature you build costs money. Every feature you don't build has opportunity cost. Without clear product direction, teams waste months building things nobody asked for while ignoring the features that would actually move the needle. The best companies in the world understand this.

## How We Can Help

### Product Strategy

<a href="https://tesla.com" target="_blank" rel="noopener noreferrer">Tesla</a> didn't become the world's most valuable automaker by building better cars. They redefined what a car could be: software-defined, constantly improving, directly sold. <a href="https://spacex.com" target="_blank" rel="noopener noreferrer">SpaceX</a> revolutionized aerospace not through incremental improvements but by questioning every assumption about rocket design. We bring that same first-principles thinking to your product strategy.

### Roadmap Development

<a href="https://boringcompany.com" target="_blank" rel="noopener noreferrer">The Boring Company</a> started with the goal of reducing tunnel costs by 10x. Every decision flows from that single metric. Your roadmap needs the same clarity. We help you identify your "one metric that matters" and ruthlessly prioritize everything against it.

### User Research

The best product teams at companies like <a href="https://gartner.com" target="_blank" rel="noopener noreferrer">Gartner</a> and <a href="https://forrester.com" target="_blank" rel="noopener noreferrer">Forrester</a> don't guess what users want. They watch users struggle. They count support tickets. They measure where people drop off. We set up the systems to capture this intelligence and turn it into roadmap decisions.

### Feature Definition

Vague requirements lead to vague products. We write specifications that answer every question before engineering starts. What does this feature do? What doesn't it do? How do we know if it worked? Clear specs mean faster builds and fewer rewrites.

### Launch Planning

Shipping isn't the finish line. It's the starting line. We plan launches that include measurement, iteration, and clear success criteria so you know within weeks (not months) whether a feature is working.

## Why This Matters

Good product management pays for itself many times over. Skip features that won't move metrics. Ship faster by eliminating ambiguity. Build confidence with stakeholders through clear communication. The best engineering team in the world can't save a product nobody wants."#.to_string(),
                    features: vec![
                        "Product strategy and vision development".to_string(),
                        "Roadmap planning and ruthless prioritization".to_string(),
                        "User research, interviews, and feedback loops".to_string(),
                        "Feature specifications that engineers love".to_string(),
                        "Sprint planning and backlog management".to_string(),
                        "Cross-functional stakeholder alignment".to_string(),
                        "Go-to-market planning and launch coordination".to_string(),
                        "Metrics definition, tracking, and iteration".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "Startup Without PM".to_string(),
                            icon: "seedling".to_string(),
                            description: "You have engineers shipping code, but nobody dedicated to making sure it's the right code".to_string(),
                        },
                        Persona {
                            title: "Overwhelmed Founder".to_string(),
                            icon: "layers".to_string(),
                            description: "You're doing product, sales, ops, and everything else, and product is the thing that keeps slipping".to_string(),
                        },
                        Persona {
                            title: "Scaling Team".to_string(),
                            icon: "bar-chart".to_string(),
                            description: "The informal processes that worked with 5 people are failing with 20, and features keep missing targets".to_string(),
                        },
                    ],
                    accent_color: "#F59E0B".to_string(), // Amber for Product
                },
                Service {
                    id: "digital-marketing".to_string(),
                    slug: "digital-marketing".to_string(),
                    title: "Digital Marketing & Customer Systems".to_string(),
                    icon: "megaphone".to_string(),
                    tagline: "Turn website visitors into customers while you sleep".to_string(),
                    description: "According to HubSpot, companies that automate lead management see a 10% or greater increase in revenue within 6-9 months. But most small businesses are still doing marketing manually, if at all. We build systems that capture leads, nurture relationships, and convert interest into revenue automatically.".to_string(),
                    long_description: r#"## The Leaky Bucket Problem

You're paying for traffic. Visitors come to your site, look around, and leave. Forever. Without a system to capture their information and follow up, you're filling a bucket with a hole in the bottom. The math is brutal: most websites convert less than 3% of visitors. That means 97% of your marketing spend walks out the door.

## How We Can Help

### Email Marketing Systems

Email marketing delivers $42 for every $1 spent, according to DMA research. But only if you actually do it. We set up systems using platforms like <a href="https://mailchimp.com" target="_blank" rel="noopener noreferrer">Mailchimp</a> for growing businesses or <a href="https://klaviyo.com" target="_blank" rel="noopener noreferrer">Klaviyo</a> for e-commerce that run automatically. Welcome sequences. Abandoned cart reminders. Re-engagement campaigns. Set it up once, let it work forever.

### SMS & Phone Outreach

Text messages have a 98% open rate (vs 20% for email). For time-sensitive offers or appointment reminders, nothing beats SMS. We integrate with <a href="https://twilio.com" target="_blank" rel="noopener noreferrer">Twilio</a> to build automated text campaigns that feel personal.

### Customer Intake & Lead Capture

Every form on your site should capture exactly what you need, route it to the right person, and trigger the right follow-up. No more leads disappearing into an unmonitored inbox. No more calling people back three days later.

### Marketing Automation

When someone downloads your pricing guide, they should get a sequence of helpful emails. When they visit your pricing page three times, your sales team should get notified. We build intelligent workflows using <a href="https://hubspot.com" target="_blank" rel="noopener noreferrer">HubSpot</a> and similar platforms that respond to customer behavior in real-time.

### Analytics & Tracking

If you can't measure it, you can't improve it. We set up <a href="https://analytics.google.com" target="_blank" rel="noopener noreferrer">Google Analytics</a> tracking that actually answers your questions: Where are leads coming from? What's working? Where are people dropping off? Data-driven decisions, not guesswork.

## The Compounding Effect

Marketing automation isn't a one-time win. It's a compounding asset. Every email sequence you build keeps working. Every lead capture form keeps collecting. Every automation keeps running. The businesses that build these systems early create an advantage that compounds over time."#.to_string(),
                    features: vec![
                        "Email marketing setup with Mailchimp or Klaviyo".to_string(),
                        "SMS campaigns and automated text sequences".to_string(),
                        "Lead capture forms that actually convert".to_string(),
                        "Appointment scheduling with automated reminders".to_string(),
                        "Membership and subscription management".to_string(),
                        "Marketing automation workflows".to_string(),
                        "CRM integration and lead routing".to_string(),
                        "Google Analytics setup and conversion tracking".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "Growing Business".to_string(),
                            icon: "seedling".to_string(),
                            description: "You've been growing through word-of-mouth and referrals, but you're ready for predictable, scalable growth".to_string(),
                        },
                        Persona {
                            title: "Service Provider".to_string(),
                            icon: "calendar".to_string(),
                            description: "No-shows and manual scheduling are costing you money and time every single week".to_string(),
                        },
                        Persona {
                            title: "Membership Business".to_string(),
                            icon: "ticket".to_string(),
                            description: "You have recurring customers, but no system to reduce churn and increase lifetime value".to_string(),
                        },
                    ],
                    accent_color: "#EC4899".to_string(), // Pink for Marketing
                },
                Service {
                    id: "technical-strategy".to_string(),
                    slug: "technical-strategy".to_string(),
                    title: "Technical Strategy & Advisory".to_string(),
                    icon: "compass".to_string(),
                    tagline: "Get the decision right before you spend the money".to_string(),
                    description: "According to Gartner, 83% of data migrations fail or exceed their budgets. The wrong technology decision can cost months of wasted effort and hundreds of thousands in sunk costs. Before you commit to a platform, sign with a vendor, or greenlight a build, you need someone in your corner who understands the technology and has no stake in the outcome.".to_string(),
                    long_description: r#"## The $500,000 Mistake

Every year, companies waste enormous budgets on technology that doesn't fit their needs. They choose vendors based on sales pitches instead of technical reality. They build custom when they should buy, or buy when they should build. These mistakes are expensive, and they're preventable.

## How We Can Help

### Technology Assessment

Firms like <a href="https://gartner.com" target="_blank" rel="noopener noreferrer">Gartner</a> and <a href="https://forrester.com" target="_blank" rel="noopener noreferrer">Forrester</a> publish research on technology trends, but their reports are written for everyone. We translate that research to your specific situation. What do you actually need? What's overkill? What's the minimum viable solution that solves your problem?

### Vendor Evaluation

Sales teams are trained to tell you what you want to hear. Their technical demos are optimized to impress, not to reveal limitations. We help you ask the hard questions: What happens when this fails? What's the real total cost of ownership? What does the migration path look like?

### Build vs Buy Analysis

The <a href="https://thoughtworks.com/radar" target="_blank" rel="noopener noreferrer">ThoughtWorks Technology Radar</a> tracks which technologies are ready for adoption and which are still experimental. But even mature technology isn't always the right choice. Sometimes custom development delivers 10x value. Sometimes off-the-shelf at 20% of the cost is good enough. We help you make that call.

### Architecture Review

Before you commit hundreds of engineering hours, have someone review the plan. We've seen the patterns that work and the patterns that fail. A few hours of review can prevent months of rework. <a href="https://ey.com" target="_blank" rel="noopener noreferrer">Ernst & Young</a> estimates that fixing architectural problems after launch costs 10-100x more than fixing them during design.

### Due Diligence

Acquiring a company or making a major technology investment? <a href="https://a16z.com" target="_blank" rel="noopener noreferrer">a16z</a> and other top VCs do technical due diligence for a reason. We assess code quality, technical debt, team capabilities, and scalability. Know what you're buying before you buy it.

## The ROI of Good Advice

One avoided mistake pays for dozens of hours of advisory work. One right decision compounds for years. The companies that invest in getting decisions right upfront outperform those that figure it out as they go."#.to_string(),
                    features: vec![
                        "Technology assessments tailored to your business".to_string(),
                        "Vendor evaluation and negotiation support".to_string(),
                        "Build vs buy analysis with total cost modeling".to_string(),
                        "Architecture reviews before you commit".to_string(),
                        "Technical due diligence for acquisitions".to_string(),
                        "Product roadmapping and prioritization".to_string(),
                        "Team capability assessment".to_string(),
                        "Documentation and knowledge transfer".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "Non-Technical Founder".to_string(),
                            icon: "graduation-cap".to_string(),
                            description: "You need someone who speaks tech and business fluently, without the cost of a full-time CTO".to_string(),
                        },
                        Persona {
                            title: "Decision Maker".to_string(),
                            icon: "scale".to_string(),
                            description: "You're about to sign a six-figure contract and want an unbiased technical opinion first".to_string(),
                        },
                        Persona {
                            title: "Acquiring Company".to_string(),
                            icon: "handshake".to_string(),
                            description: "You need to know what the technology is actually worth before you close the deal".to_string(),
                        },
                    ],
                    accent_color: "#6366F1".to_string(), // Indigo for Strategy
                },
                Service {
                    id: "business-solutions".to_string(),
                    slug: "business-solutions".to_string(),
                    title: "Business Solutions & Integration".to_string(),
                    icon: "zap".to_string(),
                    tagline: "Connect your systems, eliminate the busywork".to_string(),
                    description: "According to Zapier's State of Business Automation report, small businesses using automation save over 20 hours per week on manual tasks. That's 1,000 hours per year spent on work that adds no value. We build integrations and automations that let your team focus on work that actually matters.".to_string(),
                    long_description: r#"## The Hidden Cost of Manual Work

Every time an employee copies data from one system to another, you're paying twice: once for their time, and again in the errors they inevitably make. Research shows manual data entry has error rates of 1-4%. At scale, that's hundreds of mistakes per month, each one requiring time to find and fix.

## How We Can Help

### System Integration

<a href="https://zapier.com" target="_blank" rel="noopener noreferrer">Zapier</a> and <a href="https://make.com" target="_blank" rel="noopener noreferrer">Make</a> have made it possible to connect almost any business software without custom code. When a sale happens in your e-commerce platform, inventory updates, accounting records it, and shipping gets notified. Automatically. No copying, no pasting, no errors.

### Payment Processing

<a href="https://stripe.com" target="_blank" rel="noopener noreferrer">Stripe</a> has become the backbone of internet commerce. We integrate Stripe with your existing systems so payments flow smoothly, subscriptions manage themselves, and financial reporting happens automatically. From simple checkout to complex billing logic, we build payment workflows that work.

### Communication Automation

<a href="https://twilio.com" target="_blank" rel="noopener noreferrer">Twilio</a> powers communications for companies from startups to the Fortune 500. We use it to build automated SMS notifications, voice systems, and communication workflows that keep your customers informed without requiring your team to manually send messages.

### Custom Internal Tools

Sometimes the tool you need doesn't exist. A dashboard for your specific metrics. A calculator for your unique pricing model. An intake form that matches your exact workflow. We build internal tools that solve your specific problems without the overhead of enterprise software.

### Data Migration & Cleanup

Moving from one system to another is notoriously painful. We handle migrations so your data arrives clean, complete, and correctly mapped. No lost records. No broken relationships. No three-month cleanup project.

## The Automation Advantage

Every automation you build is an asset that keeps working forever. The hour spent building an automation that saves 5 minutes per day pays for itself in 12 days. After that, it's pure profit. The businesses that systematically eliminate manual work create compounding efficiency advantages."#.to_string(),
                    features: vec![
                        "Zapier and Make automation workflows".to_string(),
                        "Stripe payment and subscription integration".to_string(),
                        "Twilio SMS and communication automation".to_string(),
                        "Custom API integrations between systems".to_string(),
                        "Internal tools and dashboards".to_string(),
                        "Data migrations and cleanup".to_string(),
                        "Process optimization and documentation".to_string(),
                        "Ongoing maintenance and support".to_string(),
                    ],
                    personas: vec![
                        Persona {
                            title: "Operations Manager".to_string(),
                            icon: "refresh-cw".to_string(),
                            description: "You're spending more time on manual data work than on strategic initiatives".to_string(),
                        },
                        Persona {
                            title: "Growing Company".to_string(),
                            icon: "trending-up".to_string(),
                            description: "The spreadsheets and manual processes that worked at 5 employees are breaking at 20".to_string(),
                        },
                        Persona {
                            title: "Data-Driven Business".to_string(),
                            icon: "bar-chart".to_string(),
                            description: "Your data lives in 5 different places and nobody trusts any of the numbers".to_string(),
                        },
                    ],
                    accent_color: "#14B8A6".to_string(), // Teal for Integration
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
    #[cfg(target_arch = "wasm32")]
    {
        // Use JavaScript Date.now() for WASM
        let now = js_sys::Date::now() as u64;
        format!("{:x}", now)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{:x}", duration.as_millis())
    }
}

/// Get today's date in YYYY-MM-DD format
fn chrono_today() -> String {
    // Simple date - in a real app you'd use chrono crate
    "2024-01-01".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Article::generate_slug() tests ====================

    #[test]
    fn test_generate_slug_basic() {
        assert_eq!(Article::generate_slug("Hello World"), "hello-world");
    }

    #[test]
    fn test_generate_slug_with_special_characters() {
        assert_eq!(
            Article::generate_slug("What's New in 2024?"),
            "what-s-new-in-2024"
        );
    }

    #[test]
    fn test_generate_slug_with_multiple_spaces() {
        assert_eq!(
            Article::generate_slug("Too   Many    Spaces"),
            "too-many-spaces"
        );
    }

    #[test]
    fn test_generate_slug_with_leading_trailing_spaces() {
        assert_eq!(Article::generate_slug("  Trim Me  "), "trim-me");
    }

    #[test]
    fn test_generate_slug_empty_string() {
        assert_eq!(Article::generate_slug(""), "");
    }

    #[test]
    fn test_generate_slug_only_special_characters() {
        assert_eq!(Article::generate_slug("!@#$%^&*()"), "");
    }

    #[test]
    fn test_generate_slug_numbers() {
        assert_eq!(Article::generate_slug("5 Tips for 2024"), "5-tips-for-2024");
    }

    #[test]
    fn test_generate_slug_already_lowercase() {
        assert_eq!(Article::generate_slug("already-a-slug"), "already-a-slug");
    }

    #[test]
    fn test_generate_slug_mixed_case() {
        assert_eq!(Article::generate_slug("MiXeD CaSe"), "mixed-case");
    }

    #[test]
    fn test_generate_slug_unicode_preserved() {
        // Unicode alphanumeric characters are preserved by is_alphanumeric()
        assert_eq!(Article::generate_slug("Café Résumé"), "café-résumé");
    }

    #[test]
    fn test_generate_slug_punctuation() {
        assert_eq!(
            Article::generate_slug("Hello, World! How are you?"),
            "hello-world-how-are-you"
        );
    }

    // ==================== Serde Serialization Roundtrip Tests ====================

    #[test]
    fn test_article_status_serialization() {
        // Test each status variant serializes correctly
        let draft = ArticleStatus::Draft;
        let published = ArticleStatus::Published;
        let trashed = ArticleStatus::Trashed;

        assert_eq!(serde_json::to_string(&draft).unwrap(), "\"draft\"");
        assert_eq!(serde_json::to_string(&published).unwrap(), "\"published\"");
        assert_eq!(serde_json::to_string(&trashed).unwrap(), "\"trashed\"");
    }

    #[test]
    fn test_article_status_deserialization() {
        let draft: ArticleStatus = serde_json::from_str("\"draft\"").unwrap();
        let published: ArticleStatus = serde_json::from_str("\"published\"").unwrap();
        let trashed: ArticleStatus = serde_json::from_str("\"trashed\"").unwrap();

        assert_eq!(draft, ArticleStatus::Draft);
        assert_eq!(published, ArticleStatus::Published);
        assert_eq!(trashed, ArticleStatus::Trashed);
    }

    #[test]
    fn test_article_roundtrip() {
        let article = Article {
            id: "test-123".to_string(),
            title: "Test Article".to_string(),
            slug: "test-article".to_string(),
            date: "2024-01-15".to_string(),
            category: "Testing".to_string(),
            excerpt: "A test excerpt".to_string(),
            content: "Full content here".to_string(),
            status: ArticleStatus::Published,
        };

        let json = serde_json::to_string(&article).unwrap();
        let deserialized: Article = serde_json::from_str(&json).unwrap();

        assert_eq!(article, deserialized);
    }

    #[test]
    fn test_portfolio_project_roundtrip() {
        let project = PortfolioProject {
            id: "test-project".to_string(),
            slug: "test-project".to_string(),
            title: "Test Project".to_string(),
            project_type: "Website".to_string(),
            description: "A test project".to_string(),
            long_description: "Longer description".to_string(),
            external_url: "https://example.com".to_string(),
            before_url: Some("https://archive.org/example".to_string()),
            logo: Some("logo.png".to_string()),
            screenshot: None,
            video: None,
            tech_tags: vec!["Rust".to_string(), "WASM".to_string()],
            scope: vec!["Design".to_string(), "Development".to_string()],
        };

        let json = serde_json::to_string(&project).unwrap();
        let deserialized: PortfolioProject = serde_json::from_str(&json).unwrap();

        assert_eq!(project, deserialized);
    }

    #[test]
    fn test_site_settings_roundtrip() {
        let settings = SiteSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: SiteSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_discount_settings_roundtrip() {
        let discount = DiscountSettings {
            promo_discount: PromoDiscount {
                enabled: true,
                percentage: 25,
                label: Some("Holiday Sale".to_string()),
            },
            first_responder_enabled: false,
        };

        let json = serde_json::to_string(&discount).unwrap();
        let deserialized: DiscountSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(discount, deserialized);
    }

    // ==================== Default Implementation Tests ====================

    #[test]
    fn test_discount_settings_default() {
        let default = DiscountSettings::default();

        assert!(!default.promo_discount.enabled);
        assert_eq!(default.promo_discount.percentage, 10);
        assert!(default.promo_discount.label.is_none());
        assert!(default.first_responder_enabled);
    }

    #[test]
    fn test_site_settings_default_has_all_pages() {
        let settings = SiteSettings::default();

        // Verify all expected pages exist
        let page_ids: Vec<&str> = settings.pages.iter().map(|p| p.id.as_str()).collect();
        assert!(page_ids.contains(&"home"));
        assert!(page_ids.contains(&"about"));
        assert!(page_ids.contains(&"services"));
        assert!(page_ids.contains(&"portfolio"));
        assert!(page_ids.contains(&"articles"));
        assert!(page_ids.contains(&"contact"));
    }

    #[test]
    fn test_site_settings_default_feature_toggles() {
        let settings = SiteSettings::default();

        assert!(settings.features.portfolio);
        assert!(settings.features.services);
        assert!(settings.features.articles);
        assert!(settings.features.contact);
        assert!(!settings.features.testimonials); // Testimonials disabled by default
    }

    #[test]
    fn test_site_settings_default_brand() {
        let settings = SiteSettings::default();

        assert_eq!(settings.brand.name, "Pounds Consulting");
        assert!(!settings.brand.tagline.is_empty());
        assert!(settings.brand.primary_color.starts_with('#'));
        assert!(settings.brand.accent_color.starts_with('#'));
    }

    #[test]
    fn test_articles_data_default_has_articles() {
        let articles = ArticlesData::default();

        assert!(!articles.articles.is_empty());
        // All default articles should be published
        for article in &articles.articles {
            assert_eq!(article.status, ArticleStatus::Published);
        }
    }

    #[test]
    fn test_portfolio_data_default_has_projects() {
        let portfolio = PortfolioData::default();

        assert!(!portfolio.projects.is_empty());
        // Every project should have required fields
        for project in &portfolio.projects {
            assert!(!project.id.is_empty());
            assert!(!project.slug.is_empty());
            assert!(!project.title.is_empty());
            assert!(!project.external_url.is_empty());
        }
    }

    #[test]
    fn test_page_config_order_is_sequential() {
        let settings = SiteSettings::default();
        let mut orders: Vec<u32> = settings.pages.iter().map(|p| p.order).collect();
        orders.sort();

        // Orders should be 1, 2, 3, 4, 5, 6
        let expected: Vec<u32> = (1..=settings.pages.len() as u32).collect();
        assert_eq!(orders, expected);
    }

    // ==================== Article::new() Tests ====================

    #[test]
    fn test_article_new_has_empty_fields() {
        let article = Article::new();

        assert!(article.title.is_empty());
        assert!(article.slug.is_empty());
        assert!(article.excerpt.is_empty());
        assert!(article.content.is_empty());
        assert_eq!(article.status, ArticleStatus::Draft);
    }

    #[test]
    fn test_article_new_has_default_category() {
        let article = Article::new();
        assert_eq!(article.category, "General");
    }

    #[test]
    fn test_article_new_generates_id() {
        let article = Article::new();
        assert!(!article.id.is_empty());
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_portfolio_project_optional_fields() {
        let project = PortfolioProject {
            id: "minimal".to_string(),
            slug: "minimal".to_string(),
            title: "Minimal".to_string(),
            project_type: "Test".to_string(),
            description: "Desc".to_string(),
            long_description: "Long".to_string(),
            external_url: "https://example.com".to_string(),
            before_url: None,
            logo: None,
            screenshot: None,
            video: None,
            tech_tags: vec![],
            scope: vec![],
        };

        // Should serialize and deserialize correctly with None values
        let json = serde_json::to_string(&project).unwrap();
        let deserialized: PortfolioProject = serde_json::from_str(&json).unwrap();

        assert_eq!(project, deserialized);
        assert!(deserialized.before_url.is_none());
        assert!(deserialized.logo.is_none());
    }

    #[test]
    fn test_articles_data_empty() {
        let empty = ArticlesData { articles: vec![] };
        let json = serde_json::to_string(&empty).unwrap();
        let deserialized: ArticlesData = serde_json::from_str(&json).unwrap();

        assert!(deserialized.articles.is_empty());
    }

    // ==================== Sitemap Verification Tests ====================

    #[test]
    fn test_sitemap_contains_all_routes() {
        // Read sitemap.xml
        let sitemap_content =
            std::fs::read_to_string("sitemap.xml").expect("Failed to read sitemap.xml");

        // Extract all URLs from sitemap
        let sitemap_urls: Vec<&str> = sitemap_content
            .lines()
            .filter(|line| line.contains("<loc>"))
            .map(|line| {
                let start = line.find("<loc>").unwrap() + 5;
                let end = line.find("</loc>").unwrap();
                &line[start..end]
            })
            .collect();

        let base_url = "https://poundsconsulting.net";

        // Main pages that should be in sitemap
        let main_pages = vec![
            "/",
            "/about",
            "/services",
            "/portfolio",
            "/articles",
            "/contact",
        ];

        for page in main_pages {
            let full_url = if page == "/" {
                format!("{}/", base_url)
            } else {
                format!("{}{}", base_url, page)
            };
            assert!(
                sitemap_urls.contains(&full_url.as_str()),
                "Missing main page in sitemap: {}",
                full_url
            );
        }

        // Service detail pages
        let services = ServicesData::default();
        for service in &services.services {
            let url = format!("{}/services/{}", base_url, service.slug);
            assert!(
                sitemap_urls.contains(&url.as_str()),
                "Missing service page in sitemap: {}",
                url
            );
        }

        // Article detail pages (only published)
        let articles = ArticlesData::default();
        for article in articles
            .articles
            .iter()
            .filter(|a| matches!(a.status, ArticleStatus::Published))
        {
            let url = format!("{}/articles/{}", base_url, article.slug);
            assert!(
                sitemap_urls.contains(&url.as_str()),
                "Missing article page in sitemap: {}",
                url
            );
        }

        // Portfolio detail pages
        let portfolio = PortfolioData::default();
        for project in &portfolio.projects {
            let url = format!("{}/portfolio/{}", base_url, project.slug);
            assert!(
                sitemap_urls.contains(&url.as_str()),
                "Missing portfolio page in sitemap: {}",
                url
            );
        }
    }

    #[test]
    fn test_sitemap_has_no_duplicate_urls() {
        let sitemap_content =
            std::fs::read_to_string("sitemap.xml").expect("Failed to read sitemap.xml");

        let sitemap_urls: Vec<&str> = sitemap_content
            .lines()
            .filter(|line| line.contains("<loc>"))
            .map(|line| {
                let start = line.find("<loc>").unwrap() + 5;
                let end = line.find("</loc>").unwrap();
                &line[start..end]
            })
            .collect();

        let mut seen = std::collections::HashSet::new();
        for url in &sitemap_urls {
            assert!(
                seen.insert(*url),
                "Duplicate URL found in sitemap: {}",
                url
            );
        }
    }

    #[test]
    fn test_sitemap_urls_are_valid_format() {
        let sitemap_content =
            std::fs::read_to_string("sitemap.xml").expect("Failed to read sitemap.xml");

        let sitemap_urls: Vec<&str> = sitemap_content
            .lines()
            .filter(|line| line.contains("<loc>"))
            .map(|line| {
                let start = line.find("<loc>").unwrap() + 5;
                let end = line.find("</loc>").unwrap();
                &line[start..end]
            })
            .collect();

        for url in &sitemap_urls {
            assert!(
                url.starts_with("https://poundsconsulting.net"),
                "URL does not use correct base: {}",
                url
            );
            assert!(!url.contains(' '), "URL contains spaces: {}", url);
            assert!(
                !url.ends_with('/') || *url == "https://poundsconsulting.net/",
                "URL has trailing slash (except root): {}",
                url
            );
        }
    }
}

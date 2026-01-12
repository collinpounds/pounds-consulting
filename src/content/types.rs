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
                    logo: Some("assets/favicon.png".to_string()),
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
}

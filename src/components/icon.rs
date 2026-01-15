use dioxus::prelude::*;

/// Icon names available in the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconName {
    // Services
    Brain,      // AI Consulting
    Globe,      // Web Development
    Smartphone, // Mobile App Development
    Target,     // Product Management
    Megaphone,  // Digital Marketing
    Compass,    // Technical Strategy
    Zap,        // Business Solutions

    // Personas
    Briefcase,     // Business Owner
    Code,          // Tech Leader
    Settings,      // Operations Manager
    Rocket,        // Startup/New Business
    TrendingUp,    // Growing Company
    ShoppingCart,  // E-commerce
    Lightbulb,     // Startup Founder
    Building,      // Enterprise
    Wrench,        // Service Business
    Seedling,      // Small/Growing
    Layers,        // Scaling Team
    BarChart,      // Data-Driven
    Calendar,      // Service Provider
    Ticket,        // Membership
    GraduationCap, // Non-Technical
    Scale,         // Decision Maker
    Handshake,     // Acquiring Company
    RefreshCw,     // Operations/Refresh

    // Why Choose Us
    Users,         // Network
    MessageCircle, // Honest Advice
    Clock,         // Fast Response

    // General
    Check,        // Checkmark
    ArrowRight,   // Arrow
    ArrowLeft,    // Back arrow
    Star,         // Featured
    Award,        // First Responder
    Plus,         // Add
    Edit,         // Edit
    Trash,        // Delete
    Eye,          // View
    EyeOff,       // Hide
    Search,       // Search
    Menu,         // Menu
    X,            // Close
    ChevronDown,  // Dropdown
    ExternalLink, // External link
    Github,       // GitHub
    Mail,         // Email
    Phone,        // Phone
    MapPin,       // Location
    Home,         // Home
    FileText,     // Articles
    Folder,       // Portfolio
    User,         // About/Profile
    LogOut,       // Logout
    Lock,         // Password/Security
    Palette,      // Theme
    Save,         // Save
    Download,     // Download/Export
    Upload,       // Upload/Import
}

#[component]
pub fn Icon(
    name: IconName,
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor".to_string())] color: String,
    #[props(default = 2.0)] stroke_width: f32,
    #[props(default = String::new())] class: String,
) -> Element {
    let svg_content = get_icon_path(name);

    rsx! {
        svg {
            class: "icon {class}",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "{stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            dangerous_inner_html: "{svg_content}"
        }
    }
}

/// Parse a string icon name to IconName enum
pub fn parse_icon_name(name: &str) -> Option<IconName> {
    match name.to_lowercase().as_str() {
        "brain" => Some(IconName::Brain),
        "globe" => Some(IconName::Globe),
        "smartphone" => Some(IconName::Smartphone),
        "target" => Some(IconName::Target),
        "megaphone" => Some(IconName::Megaphone),
        "compass" => Some(IconName::Compass),
        "zap" => Some(IconName::Zap),
        "briefcase" => Some(IconName::Briefcase),
        "code" => Some(IconName::Code),
        "settings" => Some(IconName::Settings),
        "rocket" => Some(IconName::Rocket),
        "trending-up" | "trendingup" => Some(IconName::TrendingUp),
        "shopping-cart" | "shoppingcart" => Some(IconName::ShoppingCart),
        "lightbulb" => Some(IconName::Lightbulb),
        "building" => Some(IconName::Building),
        "wrench" => Some(IconName::Wrench),
        "seedling" => Some(IconName::Seedling),
        "layers" => Some(IconName::Layers),
        "bar-chart" | "barchart" => Some(IconName::BarChart),
        "calendar" => Some(IconName::Calendar),
        "ticket" => Some(IconName::Ticket),
        "graduation-cap" | "graduationcap" => Some(IconName::GraduationCap),
        "scale" => Some(IconName::Scale),
        "handshake" => Some(IconName::Handshake),
        "refresh-cw" | "refreshcw" => Some(IconName::RefreshCw),
        "users" => Some(IconName::Users),
        "message-circle" | "messagecircle" => Some(IconName::MessageCircle),
        "clock" => Some(IconName::Clock),
        "check" => Some(IconName::Check),
        "arrow-right" | "arrowright" => Some(IconName::ArrowRight),
        "arrow-left" | "arrowleft" => Some(IconName::ArrowLeft),
        "star" => Some(IconName::Star),
        "award" => Some(IconName::Award),
        "plus" => Some(IconName::Plus),
        "edit" => Some(IconName::Edit),
        "trash" => Some(IconName::Trash),
        "eye" => Some(IconName::Eye),
        "eye-off" | "eyeoff" => Some(IconName::EyeOff),
        "search" => Some(IconName::Search),
        "menu" => Some(IconName::Menu),
        "x" | "close" => Some(IconName::X),
        "chevron-down" | "chevrondown" => Some(IconName::ChevronDown),
        "external-link" | "externallink" => Some(IconName::ExternalLink),
        "github" => Some(IconName::Github),
        "mail" => Some(IconName::Mail),
        "phone" => Some(IconName::Phone),
        "map-pin" | "mappin" => Some(IconName::MapPin),
        "home" => Some(IconName::Home),
        "file-text" | "filetext" => Some(IconName::FileText),
        "folder" => Some(IconName::Folder),
        "user" => Some(IconName::User),
        "log-out" | "logout" => Some(IconName::LogOut),
        "lock" => Some(IconName::Lock),
        "palette" => Some(IconName::Palette),
        "save" => Some(IconName::Save),
        "download" => Some(IconName::Download),
        "upload" => Some(IconName::Upload),
        _ => None,
    }
}

fn get_icon_path(name: IconName) -> &'static str {
    match name {
        // Services
        IconName::Brain => {
            r#"<path d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z"/><path d="M12 5a3 3 0 1 1 5.997.125 4 4 0 0 1 2.526 5.77 4 4 0 0 1-.556 6.588A4 4 0 1 1 12 18Z"/><path d="M15 13a4.5 4.5 0 0 1-3-4 4.5 4.5 0 0 1-3 4"/><path d="M17.599 6.5a3 3 0 0 0 .399-1.375"/><path d="M6.003 5.125A3 3 0 0 0 6.401 6.5"/><path d="M3.477 10.896a4 4 0 0 1 .585-.396"/><path d="M19.938 10.5a4 4 0 0 1 .585.396"/><path d="M6 18a4 4 0 0 1-1.967-.516"/><path d="M19.967 17.484A4 4 0 0 1 18 18"/>"#
        }
        IconName::Globe => {
            r#"<circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/>"#
        }
        IconName::Smartphone => {
            r#"<rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/>"#
        }
        IconName::Target => {
            r#"<circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/>"#
        }
        IconName::Megaphone => {
            r#"<path d="m3 11 18-5v12L3 13v-2z"/><path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"/>"#
        }
        IconName::Compass => {
            r#"<circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/>"#
        }
        IconName::Zap => r#"<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>"#,

        // Personas
        IconName::Briefcase => {
            r#"<rect width="20" height="14" x="2" y="7" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/>"#
        }
        IconName::Code => {
            r#"<polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/>"#
        }
        IconName::Settings => {
            r#"<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>"#
        }
        IconName::Rocket => {
            r#"<path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/>"#
        }
        IconName::TrendingUp => {
            r#"<polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/><polyline points="16 7 22 7 22 13"/>"#
        }
        IconName::ShoppingCart => {
            r#"<circle cx="8" cy="21" r="1"/><circle cx="19" cy="21" r="1"/><path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12"/>"#
        }
        IconName::Lightbulb => {
            r#"<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6"/><path d="M10 22h4"/>"#
        }
        IconName::Building => {
            r#"<rect width="16" height="20" x="4" y="2" rx="2" ry="2"/><path d="M9 22v-4h6v4"/><path d="M8 6h.01"/><path d="M16 6h.01"/><path d="M12 6h.01"/><path d="M12 10h.01"/><path d="M12 14h.01"/><path d="M16 10h.01"/><path d="M16 14h.01"/><path d="M8 10h.01"/><path d="M8 14h.01"/>"#
        }
        IconName::Wrench => {
            r#"<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>"#
        }
        IconName::Seedling => {
            r#"<path d="M7 20h10"/><path d="M10 20c5.5-2.5.8-6.4 3-10"/><path d="M9.5 9.4c1.1.8 1.8 2.2 2.3 3.7-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2 2.8-.5 4.4 0 5.5.8z"/><path d="M14.1 6a7 7 0 0 0-1.1 4c1.9-.1 3.3-.6 4.3-1.4 1-1 1.6-2.3 1.7-4.6-2.7.1-4 1-4.9 2z"/>"#
        }
        IconName::Layers => {
            r#"<polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/>"#
        }
        IconName::BarChart => {
            r#"<line x1="12" x2="12" y1="20" y2="10"/><line x1="18" x2="18" y1="20" y2="4"/><line x1="6" x2="6" y1="20" y2="16"/>"#
        }
        IconName::Calendar => {
            r#"<rect width="18" height="18" x="3" y="4" rx="2" ry="2"/><line x1="16" x2="16" y1="2" y2="6"/><line x1="8" x2="8" y1="2" y2="6"/><line x1="3" x2="21" y1="10" y2="10"/>"#
        }
        IconName::Ticket => {
            r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="M13 5v2"/><path d="M13 17v2"/><path d="M13 11v2"/>"#
        }
        IconName::GraduationCap => {
            r#"<path d="M22 10v6M2 10l10-5 10 5-10 5z"/><path d="M6 12v5c3 3 9 3 12 0v-5"/>"#
        }
        IconName::Scale => {
            r#"<path d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"/><path d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"/><path d="M7 21h10"/><path d="M12 3v18"/><path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2"/>"#
        }
        IconName::Handshake => {
            r#"<path d="m11 17 2 2a1 1 0 1 0 3-3"/><path d="m14 14 2.5 2.5a1 1 0 1 0 3-3l-3.88-3.88a3 3 0 0 0-4.24 0l-.88.88a1 1 0 1 1-3-3l2.81-2.81a5.79 5.79 0 0 1 7.06-.87l.47.28a2 2 0 0 0 1.42.25L21 4"/><path d="m21 3 1 11h-2"/><path d="M3 3 2 14l6.5 6.5a1 1 0 1 0 3-3"/><path d="M3 4h8"/>"#
        }
        IconName::RefreshCw => {
            r#"<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/>"#
        }

        // Why Choose Us
        IconName::Users => {
            r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>"#
        }
        IconName::MessageCircle => r#"<path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z"/>"#,
        IconName::Clock => {
            r#"<circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>"#
        }

        // General
        IconName::Check => r#"<polyline points="20 6 9 17 4 12"/>"#,
        IconName::ArrowRight => {
            r#"<line x1="5" x2="19" y1="12" y2="12"/><polyline points="12 5 19 12 12 19"/>"#
        }
        IconName::ArrowLeft => {
            r#"<line x1="19" x2="5" y1="12" y2="12"/><polyline points="12 19 5 12 12 5"/>"#
        }
        IconName::Star => {
            r#"<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>"#
        }
        IconName::Award => {
            r#"<circle cx="12" cy="8" r="6"/><path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11"/>"#
        }
        IconName::Plus => {
            r#"<line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/>"#
        }
        IconName::Edit => {
            r#"<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"/>"#
        }
        IconName::Trash => {
            r#"<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>"#
        }
        IconName::Eye => {
            r#"<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/>"#
        }
        IconName::EyeOff => {
            r#"<path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/>"#
        }
        IconName::Search => {
            r#"<circle cx="11" cy="11" r="8"/><line x1="21" x2="16.65" y1="21" y2="16.65"/>"#
        }
        IconName::Menu => {
            r#"<line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/>"#
        }
        IconName::X => {
            r#"<line x1="18" x2="6" y1="6" y2="18"/><line x1="6" x2="18" y1="6" y2="18"/>"#
        }
        IconName::ChevronDown => r#"<polyline points="6 9 12 15 18 9"/>"#,
        IconName::ExternalLink => {
            r#"<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" x2="21" y1="14" y2="3"/>"#
        }
        IconName::Github => {
            r#"<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/><path d="M9 18c-4.51 2-5-2-7-2"/>"#
        }
        IconName::Mail => {
            r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>"#
        }
        IconName::Phone => {
            r#"<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"/>"#
        }
        IconName::MapPin => {
            r#"<path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"/><circle cx="12" cy="10" r="3"/>"#
        }
        IconName::Home => {
            r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/>"#
        }
        IconName::FileText => {
            r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="16" x2="8" y1="13" y2="13"/><line x1="16" x2="8" y1="17" y2="17"/><line x1="10" x2="8" y1="9" y2="9"/>"#
        }
        IconName::Folder => {
            r#"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/>"#
        }
        IconName::User => {
            r#"<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>"#
        }
        IconName::LogOut => {
            r#"<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" x2="9" y1="12" y2="12"/>"#
        }
        IconName::Lock => {
            r#"<rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>"#
        }
        IconName::Palette => {
            r#"<circle cx="13.5" cy="6.5" r=".5"/><circle cx="17.5" cy="10.5" r=".5"/><circle cx="8.5" cy="7.5" r=".5"/><circle cx="6.5" cy="12.5" r=".5"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.555C21.965 6.012 17.461 2 12 2z"/>"#
        }
        IconName::Save => {
            r#"<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/>"#
        }
        IconName::Download => {
            r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/>"#
        }
        IconName::Upload => {
            r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/>"#
        }
    }
}

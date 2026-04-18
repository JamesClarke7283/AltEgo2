//! Inline SVG icons.
//!
//! Paths sourced from Lucide (MIT, https://lucide.dev) unless otherwise
//! noted. All icons use `stroke="currentColor"` so they inherit any
//! Tailwind `text-*` class applied to the `<svg>` or its parent.
//!
//! Every icon uses `viewBox="0 0 24 24"` with explicit `width="24"` /
//! `height="24"` attributes. When an icon is nested inside another SVG
//! (as in the canvas node layer), the attributes size it correctly; when
//! it's used in HTML, Tailwind `w-*` / `h-*` classes override via CSS.

use leptos::prelude::*;

use crate::state::EntityType;

// --------------------------------------------------------------------------
// Entity icon dispatcher
// --------------------------------------------------------------------------

#[component]
pub fn EntityIcon(
    entity: EntityType,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    match entity {
        EntityType::AS => view! { <IconAS class=class /> }.into_any(),
        EntityType::Affiliation => view! { <IconAffiliation class=class /> }.into_any(),
        EntityType::Alias => view! { <IconAlias class=class /> }.into_any(),
        EntityType::Banner => view! { <IconBanner class=class /> }.into_any(),
        EntityType::BuiltWithRelationship => view! { <IconBuiltWithRelationship class=class /> }.into_any(),
        EntityType::BuiltWithTechnology => view! { <IconBuiltWithTechnology class=class /> }.into_any(),
        EntityType::CircularArea => view! { <IconCircularArea class=class /> }.into_any(),
        EntityType::Company => view! { <IconCompany class=class /> }.into_any(),
        EntityType::DNSName => view! { <IconDNSName class=class /> }.into_any(),
        EntityType::DateTime => view! { <IconDateTime class=class /> }.into_any(),
        EntityType::Device => view! { <IconDevice class=class /> }.into_any(),
        EntityType::Document => view! { <IconDocument class=class /> }.into_any(),
        EntityType::Domain => view! { <IconDomain class=class /> }.into_any(),
        EntityType::EmailAddress => view! { <IconEmail class=class /> }.into_any(),
        EntityType::File => view! { <IconFile class=class /> }.into_any(),
        EntityType::GPS => view! { <IconGPS class=class /> }.into_any(),
        EntityType::Hash => view! { <IconHash class=class /> }.into_any(),
        EntityType::IPv4Address => view! { <IconIPv4Address class=class /> }.into_any(),
        EntityType::Image => view! { <IconImage class=class /> }.into_any(),
        EntityType::Location => view! { <IconLocation class=class /> }.into_any(),
        EntityType::MXRecord => view! { <IconMXRecord class=class /> }.into_any(),
        EntityType::NSRecord => view! { <IconNSRecord class=class /> }.into_any(),
        EntityType::Netblock => view! { <IconNetblock class=class /> }.into_any(),
        EntityType::Organization => view! { <IconOrganization class=class /> }.into_any(),
        EntityType::Person => view! { <IconPerson class=class /> }.into_any(),
        EntityType::PhoneNumber => view! { <IconPhone class=class /> }.into_any(),
        EntityType::Phrase => view! { <IconPhrase class=class /> }.into_any(),
        EntityType::Port => view! { <IconPort class=class /> }.into_any(),
        EntityType::Sentiment => view! { <IconSentiment class=class /> }.into_any(),
        EntityType::Service => view! { <IconService class=class /> }.into_any(),
        EntityType::Twit => view! { <IconTwit class=class /> }.into_any(),
        EntityType::URL => view! { <IconURL class=class /> }.into_any(),
        EntityType::UniqueIdentifier => view! { <IconUniqueIdentifier class=class /> }.into_any(),
        EntityType::WebTitle => view! { <IconWebTitle class=class /> }.into_any(),
        EntityType::Website => view! { <IconWebsite class=class /> }.into_any(),
    }
}

// --------------------------------------------------------------------------
// Entity icons (Lucide paths unless noted). Alphabetical.
// --------------------------------------------------------------------------

#[component]
pub fn IconAS(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `network` — represents the AS (autonomous system) concept.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect x="16" y="16" width="6" height="6" rx="1"/>
            <rect x="2" y="16" width="6" height="6" rx="1"/>
            <rect x="9" y="2" width="6" height="6" rx="1"/>
            <path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"/>
            <path d="M12 12V8"/>
        </svg>
    }
}

#[component]
pub fn IconAffiliation(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `users-round` — generic social profile.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M18 21a8 8 0 0 0-16 0"/>
            <circle cx="10" cy="8" r="5"/>
            <path d="M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3"/>
        </svg>
    }
}

#[component]
pub fn IconAlias(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `venetian-mask`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M18 11c-1.5 0-2.5.5-3 2"/>
            <path d="M4 6a2 2 0 0 0-2 2v4a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V8a2 2 0 0 0-2-2h-3a8 8 0 0 0-5 2 8 8 0 0 0-5-2z"/>
            <path d="M6 11c1.5 0 2.5.5 3 2"/>
        </svg>
    }
}

#[component]
pub fn IconBanner(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `flag`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"/>
            <line x1="4" x2="4" y1="22" y2="15"/>
        </svg>
    }
}

#[component]
pub fn IconBuiltWithRelationship(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `link-2`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M9 17H7A5 5 0 0 1 7 7h2"/>
            <path d="M15 7h2a5 5 0 1 1 0 10h-2"/>
            <line x1="8" x2="16" y1="12" y2="12"/>
        </svg>
    }
}

#[component]
pub fn IconBuiltWithTechnology(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `wrench`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
        </svg>
    }
}

#[component]
pub fn IconCircularArea(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `circle-dashed`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M10.1 2.182a10 10 0 0 1 3.8 0"/>
            <path d="M13.9 21.818a10 10 0 0 1-3.8 0"/>
            <path d="M17.609 3.721a10 10 0 0 1 2.69 2.7"/>
            <path d="M2.182 13.9a10 10 0 0 1 0-3.8"/>
            <path d="M20.279 17.609a10 10 0 0 1-2.7 2.69"/>
            <path d="M21.818 10.1a10 10 0 0 1 0 3.8"/>
            <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"/>
            <path d="M6.391 20.279a10 10 0 0 1-2.69-2.7"/>
        </svg>
    }
}

#[component]
pub fn IconCompany(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `briefcase`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect width="20" height="14" x="2" y="7" rx="2" ry="2"/>
            <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/>
        </svg>
    }
}

#[component]
pub fn IconDNSName(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `globe`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/>
            <path d="M2 12h20"/>
        </svg>
    }
}

#[component]
pub fn IconDateTime(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `calendar-clock`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5"/>
            <path d="M16 2v4"/>
            <path d="M8 2v4"/>
            <path d="M3 10h5"/>
            <path d="M17.5 17.5 16 16.3V14"/>
            <circle cx="16" cy="16" r="6"/>
        </svg>
    }
}

#[component]
pub fn IconDevice(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `monitor`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect width="20" height="14" x="2" y="3" rx="2"/>
            <line x1="8" x2="16" y1="21" y2="21"/>
            <line x1="12" x2="12" y1="17" y2="21"/>
        </svg>
    }
}

#[component]
pub fn IconDocument(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `file-text`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/>
            <path d="M14 2v4a2 2 0 0 0 2 2h4"/>
            <path d="M10 9H8"/>
            <path d="M16 13H8"/>
            <path d="M16 17H8"/>
        </svg>
    }
}

#[component]
pub fn IconDomain(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `globe-2` — globe with horizontal bands.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M21.54 15H17a2 2 0 0 0-2 2v4.54"/>
            <path d="M7 3.34V5a3 3 0 0 0 3 3v0a2 2 0 0 1 2 2v0c0 1.1.9 2 2 2v0a2 2 0 0 0 2-2v0c0-1.1.9-2 2-2h3.17"/>
            <path d="M11 21.95V18a2 2 0 0 0-2-2v0a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05"/>
            <circle cx="12" cy="12" r="10"/>
        </svg>
    }
}

#[component]
pub fn IconEmail(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect x="2" y="4" width="20" height="16" rx="2"/>
            <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>
        </svg>
    }
}

#[component]
pub fn IconFile(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `file`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/>
            <path d="M14 2v4a2 2 0 0 0 2 2h4"/>
        </svg>
    }
}

#[component]
pub fn IconGPS(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `map-pin` with an inner crosshair to hint GPS.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <circle cx="12" cy="12" r="9"/>
            <path d="M12 3v2"/><path d="M12 19v2"/>
            <path d="M3 12h2"/><path d="M19 12h2"/>
            <circle cx="12" cy="12" r="2" fill="currentColor" stroke="none"/>
        </svg>
    }
}

#[component]
pub fn IconHash(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `hash`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <line x1="4" x2="20" y1="9" y2="9"/>
            <line x1="4" x2="20" y1="15" y2="15"/>
            <line x1="10" x2="8" y1="3" y2="21"/>
            <line x1="16" x2="14" y1="3" y2="21"/>
        </svg>
    }
}

#[component]
pub fn IconIPv4Address(#[prop(into, optional)] class: String) -> impl IntoView {
    // Hand-drawn: square with dotted-quad implication.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect x="3" y="6" width="18" height="12" rx="2"/>
            <circle cx="8" cy="12" r="0.5" fill="currentColor"/>
            <circle cx="12" cy="12" r="0.5" fill="currentColor"/>
            <circle cx="16" cy="12" r="0.5" fill="currentColor"/>
            <path d="M6 10v4"/>
            <path d="M18 10v4"/>
        </svg>
    }
}

#[component]
pub fn IconImage(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `image`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect width="18" height="18" x="3" y="3" rx="2" ry="2"/>
            <circle cx="9" cy="9" r="2"/>
            <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/>
        </svg>
    }
}

#[component]
pub fn IconLocation(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `map-pin`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M20 10c0 7-8 13-8 13s-8-6-8-13a8 8 0 0 1 16 0Z"/>
            <circle cx="12" cy="10" r="3"/>
        </svg>
    }
}

#[component]
pub fn IconMXRecord(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `mailbox`-inspired.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M22 17a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v2h20z"/>
            <path d="M15 15V9a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v6"/>
            <path d="M2 9h7"/>
            <path d="M5 9V7"/>
            <text x="4" y="14" font-family="monospace" font-size="5" fill="currentColor" stroke="none">"MX"</text>
        </svg>
    }
}

#[component]
pub fn IconNSRecord(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `server`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
            <rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
            <line x1="6" x2="6.01" y1="6" y2="6"/>
            <line x1="6" x2="6.01" y1="18" y2="18"/>
        </svg>
    }
}

#[component]
pub fn IconNetblock(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `layers`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z"/>
            <path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65"/>
            <path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65"/>
        </svg>
    }
}

#[component]
pub fn IconOrganization(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `building-2` (simplified).
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z"/>
            <path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"/>
            <path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2"/>
            <path d="M10 6h4"/>
            <path d="M10 10h4"/>
            <path d="M10 14h4"/>
            <path d="M10 18h4"/>
        </svg>
    }
}

#[component]
pub fn IconPerson(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
        </svg>
    }
}

#[component]
pub fn IconPhone(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.86 19.86 0 0 1-8.63-3.07 19.55 19.55 0 0 1-6-6 19.86 19.86 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.12.9.31 1.78.58 2.64a2 2 0 0 1-.45 2.11L8 9.71a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.86.27 1.74.46 2.64.58A2 2 0 0 1 22 16.92z"/>
        </svg>
    }
}

#[component]
pub fn IconPhrase(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `quote`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M16 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/>
            <path d="M5 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/>
        </svg>
    }
}

#[component]
pub fn IconPort(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `plug`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M12 22v-5"/>
            <path d="M9 7V2"/>
            <path d="M15 7V2"/>
            <path d="M6 13V8a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4z"/>
        </svg>
    }
}

#[component]
pub fn IconSentiment(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `smile`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <circle cx="12" cy="12" r="10"/>
            <path d="M8 14s1.5 2 4 2 4-2 4-2"/>
            <line x1="9" x2="9.01" y1="9" y2="9"/>
            <line x1="15" x2="15.01" y1="9" y2="9"/>
        </svg>
    }
}

#[component]
pub fn IconService(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `cpu`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect x="4" y="4" width="16" height="16" rx="2"/>
            <rect x="9" y="9" width="6" height="6"/>
            <path d="M15 2v2"/><path d="M15 20v2"/>
            <path d="M2 15h2"/><path d="M2 9h2"/>
            <path d="M20 15h2"/><path d="M20 9h2"/>
            <path d="M9 2v2"/><path d="M9 20v2"/>
        </svg>
    }
}

#[component]
pub fn IconTwit(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `message-square`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
        </svg>
    }
}

#[component]
pub fn IconURL(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `link`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
        </svg>
    }
}

#[component]
pub fn IconUniqueIdentifier(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `fingerprint`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M12 11V6a5 5 0 0 1 5-5"/>
            <path d="M3 11a9 9 0 0 1 14-7.5"/>
            <path d="M12 11a3 3 0 0 1 3 3v5"/>
            <path d="M3 15a13 13 0 0 0 3 4"/>
            <path d="M12 15a9 9 0 0 0 2 6"/>
            <path d="M19 11v3a7 7 0 0 1-1 4"/>
            <path d="M7 19a7 7 0 0 1-3-6"/>
        </svg>
    }
}

#[component]
pub fn IconWebTitle(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `type` — glyphic "T" for text-title.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <polyline points="4 7 4 4 20 4 20 7"/>
            <line x1="9" x2="15" y1="20" y2="20"/>
            <line x1="12" x2="12" y1="4" y2="20"/>
        </svg>
    }
}

#[component]
pub fn IconWebsite(#[prop(into, optional)] class: String) -> impl IntoView {
    // Hand-drawn globe inside browser frame.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect x="2" y="4" width="20" height="16" rx="2"/>
            <path d="M2 9h20"/>
            <circle cx="5" cy="6.5" r="0.5" fill="currentColor"/>
            <circle cx="7.5" cy="6.5" r="0.5" fill="currentColor"/>
            <circle cx="10" cy="6.5" r="0.5" fill="currentColor"/>
            <circle cx="12" cy="14.5" r="3.5"/>
            <path d="M12 11a7 7 0 0 0 0 7 7 7 0 0 0 0-7"/>
        </svg>
    }
}

// --------------------------------------------------------------------------
// UI chrome icons
// --------------------------------------------------------------------------

#[component]
pub fn IconPlus(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M5 12h14"/><path d="M12 5v14"/>
        </svg>
    }
}

#[component]
pub fn IconMinus(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M5 12h14"/>
        </svg>
    }
}

#[component]
pub fn IconMaximizeBox(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `maximize`, used for the Fit-to-view button.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M8 3H5a2 2 0 0 0-2 2v3"/>
            <path d="M21 8V5a2 2 0 0 0-2-2h-3"/>
            <path d="M3 16v3a2 2 0 0 0 2 2h3"/>
            <path d="M16 21h3a2 2 0 0 0 2-2v-3"/>
        </svg>
    }
}

#[component]
pub fn IconLock(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect width="18" height="11" x="3" y="11" rx="2" ry="2"/>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
        </svg>
    }
}

#[component]
pub fn IconLockOpen(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect width="18" height="11" x="3" y="11" rx="2" ry="2"/>
            <path d="M7 11V7a5 5 0 0 1 9.9-1"/>
        </svg>
    }
}

#[component]
pub fn IconSunburst(#[prop(into, optional)] class: String) -> impl IntoView {
    // The orange spiky "brand" glyph.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <circle cx="12" cy="12" r="3.5" fill="currentColor" stroke="none"/>
            <path d="M12 2v3"/><path d="M12 19v3"/>
            <path d="M2 12h3"/><path d="M19 12h3"/>
            <path d="M4.93 4.93 7.05 7.05"/>
            <path d="m16.95 16.95 2.12 2.12"/>
            <path d="M4.93 19.07 7.05 16.95"/>
            <path d="m16.95 7.05 2.12-2.12"/>
        </svg>
    }
}

#[component]
pub fn IconSpiderWeb(#[prop(into, optional)] class: String) -> impl IntoView {
    // Hand-rolled spider-web for the empty canvas state.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"
            width="100" height="100" fill="none"
            stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <circle cx="50" cy="50" r="42" stroke-dasharray="3 3"/>
            <circle cx="50" cy="50" r="30"/>
            <circle cx="50" cy="50" r="18"/>
            <circle cx="50" cy="50" r="8"/>
            <path d="M50 8 V92"/>
            <path d="M8 50 H92"/>
            <path d="M20 20 L80 80"/>
            <path d="M80 20 L20 80"/>
        </svg>
    }
}

#[component]
pub fn IconWinMinimize(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M5 12h14"/>
        </svg>
    }
}

#[component]
pub fn IconWinMaximize(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <rect x="5" y="5" width="14" height="14" rx="1"/>
        </svg>
    }
}

#[component]
pub fn IconWinClose(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
        </svg>
    }
}

#[component]
pub fn IconCheck(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <path d="M20 6 9 17l-5-5"/>
        </svg>
    }
}

#[component]
pub fn IconSearch(#[prop(into, optional)] class: String) -> impl IntoView {
    // Lucide `search`.
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width="24" height="24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class=class>
            <circle cx="11" cy="11" r="8"/>
            <path d="m21 21-4.3-4.3"/>
        </svg>
    }
}

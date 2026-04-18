//! Global application state.
//!
//! A single `AppState` is put into `provide_context` at the `<App/>` root and
//! accessed by every component. All reactive pieces live on `RwSignal`s so
//! updates propagate without prop-drilling.

use indexmap::IndexMap;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Entity types  — 35 variants matching Maltego's Standard entity set.
// Reference: https://github.com/dreadl0ck/maltego/blob/main/entities.go
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    AS,
    Affiliation,
    Alias,
    Banner,
    BuiltWithRelationship,
    BuiltWithTechnology,
    CircularArea,
    Company,
    DNSName,
    DateTime,
    Device,
    Document,
    Domain,
    EmailAddress,
    File,
    GPS,
    Hash,
    IPv4Address,
    Image,
    Location,
    MXRecord,
    NSRecord,
    Netblock,
    /// Renamed from `Organisation` (British) → `Organization` (American,
    /// matching Maltego). The serde alias keeps already-saved files loadable.
    #[serde(alias = "Organisation")]
    Organization,
    Person,
    PhoneNumber,
    Phrase,
    Port,
    Sentiment,
    Service,
    Twit,
    #[serde(rename = "URL")]
    URL,
    UniqueIdentifier,
    WebTitle,
    Website,
}

impl EntityType {
    pub const ALL: [EntityType; 35] = [
        EntityType::AS,
        EntityType::Affiliation,
        EntityType::Alias,
        EntityType::Banner,
        EntityType::BuiltWithRelationship,
        EntityType::BuiltWithTechnology,
        EntityType::CircularArea,
        EntityType::Company,
        EntityType::DNSName,
        EntityType::DateTime,
        EntityType::Device,
        EntityType::Document,
        EntityType::Domain,
        EntityType::EmailAddress,
        EntityType::File,
        EntityType::GPS,
        EntityType::Hash,
        EntityType::IPv4Address,
        EntityType::Image,
        EntityType::Location,
        EntityType::MXRecord,
        EntityType::NSRecord,
        EntityType::Netblock,
        EntityType::Organization,
        EntityType::Person,
        EntityType::PhoneNumber,
        EntityType::Phrase,
        EntityType::Port,
        EntityType::Sentiment,
        EntityType::Service,
        EntityType::Twit,
        EntityType::URL,
        EntityType::UniqueIdentifier,
        EntityType::WebTitle,
        EntityType::Website,
    ];

    pub fn label(self) -> &'static str {
        match self {
            EntityType::AS => "AS",
            EntityType::Affiliation => "Affiliation",
            EntityType::Alias => "Alias",
            EntityType::Banner => "Banner",
            EntityType::BuiltWithRelationship => "Built With Relationship",
            EntityType::BuiltWithTechnology => "Built With Technology",
            EntityType::CircularArea => "Circular Area",
            EntityType::Company => "Company",
            EntityType::DNSName => "DNS Name",
            EntityType::DateTime => "Date / Time",
            EntityType::Device => "Device",
            EntityType::Document => "Document",
            EntityType::Domain => "Domain",
            EntityType::EmailAddress => "Email Address",
            EntityType::File => "File",
            EntityType::GPS => "GPS",
            EntityType::Hash => "Hash",
            EntityType::IPv4Address => "IPv4 Address",
            EntityType::Image => "Image",
            EntityType::Location => "Location",
            EntityType::MXRecord => "MX Record",
            EntityType::NSRecord => "NS Record",
            EntityType::Netblock => "Netblock",
            EntityType::Organization => "Organization",
            EntityType::Person => "Person",
            EntityType::PhoneNumber => "Phone Number",
            EntityType::Phrase => "Phrase",
            EntityType::Port => "Port",
            EntityType::Sentiment => "Sentiment",
            EntityType::Service => "Service",
            EntityType::Twit => "Twit",
            EntityType::URL => "URL",
            EntityType::UniqueIdentifier => "Unique Identifier",
            EntityType::WebTitle => "Web Title",
            EntityType::Website => "Website",
        }
    }

    /// Stable snake_case slug used in the drag-drop dataTransfer. Distinct
    /// from the serde JSON representation (which uses the variant name
    /// verbatim).
    pub fn as_str(self) -> &'static str {
        match self {
            EntityType::AS => "as",
            EntityType::Affiliation => "affiliation",
            EntityType::Alias => "alias",
            EntityType::Banner => "banner",
            EntityType::BuiltWithRelationship => "built_with_relationship",
            EntityType::BuiltWithTechnology => "built_with_technology",
            EntityType::CircularArea => "circular_area",
            EntityType::Company => "company",
            EntityType::DNSName => "dns_name",
            EntityType::DateTime => "datetime",
            EntityType::Device => "device",
            EntityType::Document => "document",
            EntityType::Domain => "domain",
            EntityType::EmailAddress => "email_address",
            EntityType::File => "file",
            EntityType::GPS => "gps",
            EntityType::Hash => "hash",
            EntityType::IPv4Address => "ipv4_address",
            EntityType::Image => "image",
            EntityType::Location => "location",
            EntityType::MXRecord => "mx_record",
            EntityType::NSRecord => "ns_record",
            EntityType::Netblock => "netblock",
            EntityType::Organization => "organization",
            EntityType::Person => "person",
            EntityType::PhoneNumber => "phone_number",
            EntityType::Phrase => "phrase",
            EntityType::Port => "port",
            EntityType::Sentiment => "sentiment",
            EntityType::Service => "service",
            EntityType::Twit => "twit",
            EntityType::URL => "url",
            EntityType::UniqueIdentifier => "unique_identifier",
            EntityType::WebTitle => "web_title",
            EntityType::Website => "website",
        }
    }

    pub fn from_str(s: &str) -> Option<EntityType> {
        Some(match s {
            "as" => EntityType::AS,
            "affiliation" => EntityType::Affiliation,
            "alias" => EntityType::Alias,
            "banner" => EntityType::Banner,
            "built_with_relationship" => EntityType::BuiltWithRelationship,
            "built_with_technology" => EntityType::BuiltWithTechnology,
            "circular_area" => EntityType::CircularArea,
            "company" => EntityType::Company,
            "dns_name" => EntityType::DNSName,
            "datetime" => EntityType::DateTime,
            "device" => EntityType::Device,
            "document" => EntityType::Document,
            "domain" => EntityType::Domain,
            "email_address" => EntityType::EmailAddress,
            "file" => EntityType::File,
            "gps" => EntityType::GPS,
            "hash" => EntityType::Hash,
            "ipv4_address" => EntityType::IPv4Address,
            "image" => EntityType::Image,
            "location" => EntityType::Location,
            "mx_record" => EntityType::MXRecord,
            "ns_record" => EntityType::NSRecord,
            "netblock" => EntityType::Netblock,
            "organization" | "organisation" => EntityType::Organization,
            "person" => EntityType::Person,
            "phone_number" => EntityType::PhoneNumber,
            "phrase" => EntityType::Phrase,
            "port" => EntityType::Port,
            "sentiment" => EntityType::Sentiment,
            "service" => EntityType::Service,
            "twit" => EntityType::Twit,
            "url" => EntityType::URL,
            "unique_identifier" => EntityType::UniqueIdentifier,
            "web_title" => EntityType::WebTitle,
            "website" => EntityType::Website,
            _ => return None,
        })
    }

    /// Starter property fields for a newly-dropped node. Values default to
    /// empty strings and are editable in the right sidebar property view.
    pub fn default_properties(self) -> IndexMap<String, String> {
        let keys: &[&str] = match self {
            EntityType::AS => &["AS Number", "Description", "Notes"],
            EntityType::Affiliation => &["Name", "Network", "Profile URL", "Notes"],
            EntityType::Alias => &["Handle", "Platform", "First Seen", "Notes"],
            EntityType::Banner => &["Banner", "Service", "Port", "Notes"],
            EntityType::BuiltWithRelationship => &["Name", "Category", "Notes"],
            EntityType::BuiltWithTechnology => &["Technology", "Category", "Version", "Notes"],
            EntityType::CircularArea => &["Name", "Latitude", "Longitude", "Radius"],
            EntityType::Company => &["Name", "Domain", "Country", "Industry", "Notes"],
            EntityType::DNSName => &["Name", "Record Type", "Value", "Notes"],
            EntityType::DateTime => &["Timestamp", "Timezone", "Description"],
            EntityType::Device => &["Name", "Type", "OS", "Notes"],
            EntityType::Document => &["Title", "URL", "Hash", "Notes"],
            EntityType::Domain => &["Domain", "Registrar", "Created", "Expires", "Notes"],
            EntityType::EmailAddress => &["Address", "Domain", "Verified", "Source", "Notes"],
            EntityType::File => &["File Name", "Path", "Size", "Hash"],
            EntityType::GPS => &["Latitude", "Longitude", "Altitude", "Accuracy"],
            EntityType::Hash => &["Value", "Algorithm", "Source", "Notes"],
            EntityType::IPv4Address => &["Address", "Owner", "Country", "ASN", "Notes"],
            EntityType::Image => &["Title", "URL", "Width", "Height", "Notes"],
            EntityType::Location => &["Name", "City", "Country", "Latitude", "Longitude"],
            EntityType::MXRecord => &["Value", "Priority", "Domain", "Notes"],
            EntityType::NSRecord => &["Value", "Domain", "Notes"],
            EntityType::Netblock => &["CIDR", "Owner", "Country", "Notes"],
            EntityType::Organization => &["Name", "Domain", "Country", "Industry", "Notes"],
            // Person no longer seeds an "Aliases" field — use an Alias node.
            EntityType::Person => &["Full Name", "Date of Birth", "Location", "Notes"],
            EntityType::PhoneNumber => &["Number", "Country", "Carrier", "Type", "Notes"],
            EntityType::Phrase => &["Text", "Language", "Source", "Notes"],
            EntityType::Port => &["Number", "Protocol", "Service", "State"],
            EntityType::Sentiment => &["Text", "Score", "Polarity", "Notes"],
            EntityType::Service => &["Name", "Port", "Version", "Banner"],
            EntityType::Twit => &["Handle", "Content", "URL", "Date"],
            EntityType::URL => &["URL", "Title", "Status Code", "Notes"],
            EntityType::UniqueIdentifier => &["Value", "Kind", "Source", "Notes"],
            EntityType::WebTitle => &["Title", "URL", "Notes"],
            EntityType::Website => &["Domain", "URL", "Technology", "Status", "Notes"],
        };
        keys.iter().map(|k| (k.to_string(), String::new())).collect()
    }
}

// ---------------------------------------------------------------------------
// Graph primitives
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeId(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EdgeId(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub entity_type: EntityType,
    pub position: (f64, f64),
    pub properties: IndexMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    #[serde(default)]
    pub label: Option<String>,
}

// ---------------------------------------------------------------------------
// Viewport / theme / selection / drag
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub pan: (f64, f64),
    pub zoom: f64,
    pub locked: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            pan: (0.0, 0.0),
            zoom: 1.0,
            locked: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Option<Theme> {
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }

    pub fn toggle(self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    None,
    Node(NodeId),
    Edge(EdgeId),
}

#[derive(Copy, Clone, Debug)]
pub enum DragKind {
    None,
    PanCanvas {
        start_pan: (f64, f64),
        start_client: (f64, f64),
    },
    MoveNode {
        id: NodeId,
        offset: (f64, f64),
    },
    NewEdge {
        from: NodeId,
        cursor_world: (f64, f64),
    },
}

// ---------------------------------------------------------------------------
// AppState
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct AppState {
    pub nodes: RwSignal<IndexMap<NodeId, Node>>,
    pub edges: RwSignal<IndexMap<EdgeId, Edge>>,
    pub selection: RwSignal<Selection>,
    pub viewport: RwSignal<Viewport>,
    pub theme: RwSignal<Theme>,
    pub drag: RwSignal<DragKind>,
    pub menu_open: RwSignal<Option<&'static str>>,
    pub hovered_node: RwSignal<Option<NodeId>>,
    pub next_node_id: RwSignal<u64>,
    pub next_edge_id: RwSignal<u64>,
    /// Absolute path to the currently-open `.altego.json` file, or `None`
    /// when the graph has never been saved. `File → Save` writes to this
    /// path silently when it's `Some`; `Save As…` always reassigns it.
    pub current_file_path: RwSignal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            nodes: RwSignal::new(IndexMap::new()),
            edges: RwSignal::new(IndexMap::new()),
            selection: RwSignal::new(Selection::None),
            viewport: RwSignal::new(Viewport::default()),
            theme: RwSignal::new(Theme::Dark),
            drag: RwSignal::new(DragKind::None),
            menu_open: RwSignal::new(None),
            hovered_node: RwSignal::new(None),
            next_node_id: RwSignal::new(1),
            next_edge_id: RwSignal::new(1),
            current_file_path: RwSignal::new(None),
        }
    }

    /// Retrieve the shared `AppState` from context. Panics if called outside
    /// the `<App/>` subtree (which should never happen in practice).
    pub fn expect() -> Self {
        expect_context::<AppState>()
    }

    // --------- graph mutations ---------

    pub fn add_node(&self, entity_type: EntityType, position: (f64, f64)) -> NodeId {
        let id = NodeId(self.next_node_id.get_untracked());
        self.next_node_id.update(|n| *n += 1);
        let node = Node {
            id,
            entity_type,
            position,
            properties: entity_type.default_properties(),
        };
        self.nodes.update(|m| {
            m.insert(id, node);
        });
        self.selection.set(Selection::Node(id));
        id
    }

    pub fn add_edge(&self, from: NodeId, to: NodeId) -> Option<EdgeId> {
        if from == to {
            return None;
        }
        // Ignore duplicate edges (either direction).
        let exists = self.edges.with_untracked(|edges| {
            edges.values().any(|e| {
                (e.from == from && e.to == to) || (e.from == to && e.to == from)
            })
        });
        if exists {
            return None;
        }
        let id = EdgeId(self.next_edge_id.get_untracked());
        self.next_edge_id.update(|n| *n += 1);
        self.edges.update(|m| {
            m.insert(id, Edge { id, from, to, label: None });
        });
        Some(id)
    }

    pub fn clear(&self) {
        self.nodes.update(|n| n.clear());
        self.edges.update(|e| e.clear());
        self.selection.set(Selection::None);
        self.viewport.set(Viewport::default());
        self.drag.set(DragKind::None);
        self.current_file_path.set(None);
    }

    pub fn toggle_theme(&self) {
        self.theme.update(|t| *t = t.toggle());
    }

    pub fn toggle_lock(&self) {
        self.viewport.update(|v| v.locked = !v.locked);
    }

    // --------- persistence ---------

    /// Build a serialisable snapshot of the current graph (no viewport,
    /// selection, theme — those are UI state).
    pub fn snapshot_graph_file(&self) -> GraphFile {
        GraphFile {
            version: GRAPH_FILE_VERSION,
            nodes: self.nodes.with_untracked(|m| m.values().cloned().collect()),
            edges: self.edges.with_untracked(|m| m.values().cloned().collect()),
        }
    }

    /// Replace all graph data with contents of a loaded file. Bumps the
    /// next-id counters past the highest loaded ids so future inserts stay
    /// unique. Does NOT touch `current_file_path` — the caller should set
    /// that after a successful Open.
    pub fn load_graph_file(&self, gf: GraphFile) {
        let mut nodes = IndexMap::new();
        let mut max_node_id = 0u64;
        for n in gf.nodes {
            max_node_id = max_node_id.max(n.id.0);
            nodes.insert(n.id, n);
        }
        let mut edges = IndexMap::new();
        let mut max_edge_id = 0u64;
        for e in gf.edges {
            max_edge_id = max_edge_id.max(e.id.0);
            edges.insert(e.id, e);
        }
        self.nodes.set(nodes);
        self.edges.set(edges);
        self.next_node_id.set(max_node_id + 1);
        self.next_edge_id.set(max_edge_id + 1);
        self.selection.set(Selection::None);
        self.viewport.set(Viewport::default());
        self.drag.set(DragKind::None);
    }
}

// ---------------------------------------------------------------------------
// On-disk file format
// ---------------------------------------------------------------------------

/// Bump this on breaking schema changes.
pub const GRAPH_FILE_VERSION: u32 = 1;

/// Serialisable contents of a `.altego.json` file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphFile {
    pub version: u32,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

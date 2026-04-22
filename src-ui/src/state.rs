//! Global application state.
//!
//! A single `AppState` is put into `provide_context` at the `<App/>` root and
//! accessed by every component. All reactive pieces live on `RwSignal`s so
//! updates propagate without prop-drilling.

use std::collections::VecDeque;

use indexmap::IndexMap;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Maximum depth of the undo/redo stacks. 100 is plenty for interactive
/// editing and caps memory at a few MB even for large graphs.
const MAX_UNDO_STACK: usize = 100;

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
// Gadgets (right-click "transforms" on nodes).
// ---------------------------------------------------------------------------

/// Where the right-click context menu is anchored and which node owns it.
/// `screen_pos` is *viewport* coordinates (what `MouseEvent::client_x/y`
/// returns), not world coordinates, because the menu is a `position:fixed`
/// overlay outside the SVG viewport transform.
#[derive(Clone, Debug)]
pub struct ContextMenuState {
    pub node_id: NodeId,
    pub entity_type: EntityType,
    pub screen_pos: (f64, f64),
}

/// One in-flight (or completed) gadget run.
///
/// **Reactivity design.** Every field that changes during a run (progress
/// counters, result list, status counts, spawned-nodes map, terminal
/// flag) is stored as its own `RwSignal` — *not* as a plain value on the
/// struct. Consequences:
///
///   * Pushing a result into `results` only invalidates the `results`
///     signal. Code that only reads, say, `title` does **not** re-run.
///   * The outer `AppState.gadget_runs: RwSignal<IndexMap<_, GadgetRun>>`
///     only needs to fire on *structural* changes (a run is created or
///     removed), not on every one of the ~3 000 progress events per run.
///   * `GadgetRun: Clone` is cheap — signals are integer handles, so the
///     whole struct is a handful of IDs + two `String`s.
///
/// Before this split, each progress tick fanned out to every observer of
/// `gadget_runs`, which did O(N) work per tick and produced visible UI
/// jank on ~3 000-site sweeps.
#[derive(Clone, Debug)]
pub struct GadgetRun {
    pub run_id: String,
    pub title: String,
    /// The node the gadget was launched from (the Alias). Used as the
    /// connection target when the user clicks a result to materialise it
    /// as a child node.
    pub source_node_id: NodeId,
    /// Completed / total site counters. Drive the progress bar.
    pub completed: RwSignal<usize>,
    pub total: RwSignal<usize>,
    /// Append-only log of every site result that has arrived.
    pub results: RwSignal<Vec<gadgets_maigret::SiteCheckResult>>,
    /// Cached per-status counts — incrementally updated as each result
    /// lands. Read by the filter pills in O(1).
    pub counts: RwSignal<gadgets_maigret::StatusCounts>,
    /// Per-run map: `site name` → `NodeId` for every claimed result the
    /// user clicked to spawn as a graph node. Click-again removes the
    /// node and its edge, then drops the entry. If the user deletes the
    /// child node manually on the canvas we detect the stale `NodeId` on
    /// next toggle and fall through to re-add.
    pub spawned_nodes: RwSignal<IndexMap<String, NodeId>>,
    /// `true` once the backend command resolved. Either the final `results`
    /// Vec is set (Ok) or `error` is set (Err).
    pub finished: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

// ---------------------------------------------------------------------------
// Undo / redo snapshots
// ---------------------------------------------------------------------------

/// An immutable snapshot of the graph. Cheap enough to stash in the
/// undo/redo stacks even for thousand-node graphs (a single clone of two
/// `IndexMap`s + a few scalars).
///
/// `viewport`, `theme`, `current_file_path`, and everything gadget-related
/// are **not** included — undo is about graph structure, not cosmetic UI
/// state.
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub nodes: IndexMap<NodeId, Node>,
    pub edges: IndexMap<EdgeId, Edge>,
    pub selection: Selection,
    pub next_node_id: u64,
    pub next_edge_id: u64,
    /// Dirty flag at snapshot time — so undo back past a Save restores
    /// the "green dot" state, and redo forward into an edit restores the
    /// "white dot" state.
    pub is_dirty: bool,
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
    // --------- gadgets (right-click actions on nodes) ---------
    /// The node-level right-click menu. `Some` while the menu is open.
    pub context_menu: RwSignal<Option<ContextMenuState>>,
    /// Every gadget run ever spawned in this session, keyed by run_id.
    /// Kept so the user can re-open a previous run after minimising it.
    pub gadget_runs: RwSignal<IndexMap<String, GadgetRun>>,
    /// Which gadget run the bottom-right results panel is displaying.
    /// `None` = panel hidden.
    pub active_gadget_run: RwSignal<Option<String>>,
    // --------- dirty / save state ---------
    /// `true` when the in-memory graph has unsaved changes — i.e. it
    /// differs from whatever's on disk at `current_file_path` (or, for a
    /// brand-new untitled graph, differs from "nothing").
    ///
    /// Set by mutation helpers (`add_node`, `remove_node`, …), the node-
    /// drag start, and right-sidebar property edits. Cleared on
    /// successful save / open / new. Captured in `Snapshot` so
    /// undo/redo restore the right dirty flag.
    pub is_dirty: RwSignal<bool>,
    // --------- undo / redo ---------
    /// Previous graph states, most-recent last. `push_undo_snapshot()` is
    /// called at the start of every mutation that participates in undo;
    /// `undo()` pops from here and applies.
    pub undo_stack: RwSignal<VecDeque<Snapshot>>,
    /// States rolled away by `undo()`, most-recently-undone last.
    /// `redo()` pops from here. Cleared whenever a new mutation happens.
    pub redo_stack: RwSignal<VecDeque<Snapshot>>,
    /// Non-zero while a multi-step action is in progress — inner
    /// mutations skip their own snapshot so the whole action is one
    /// undo. Use [`AppState::begin_transaction`] / `end_transaction` to
    /// drive this.
    pub transaction_depth: RwSignal<usize>,
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
            context_menu: RwSignal::new(None),
            gadget_runs: RwSignal::new(IndexMap::new()),
            active_gadget_run: RwSignal::new(None),
            is_dirty: RwSignal::new(false),
            undo_stack: RwSignal::new(VecDeque::new()),
            redo_stack: RwSignal::new(VecDeque::new()),
            transaction_depth: RwSignal::new(0),
        }
    }

    /// Retrieve the shared `AppState` from context. Panics if called outside
    /// the `<App/>` subtree (which should never happen in practice).
    pub fn expect() -> Self {
        expect_context::<AppState>()
    }

    // --------- dirty / save state ---------

    /// Mark the graph as having unsaved changes. Idempotent: if the flag
    /// is already `true`, no signal fire — avoids redundant reactive
    /// work during long drags / keystroke streams.
    pub fn mark_dirty(&self) {
        if !self.is_dirty.get_untracked() {
            self.is_dirty.set(true);
        }
    }

    /// Mark the graph as in-sync with disk. Called by save / load / new.
    pub fn mark_clean(&self) {
        if self.is_dirty.get_untracked() {
            self.is_dirty.set(false);
        }
    }

    // --------- undo / redo ---------

    /// Capture the current graph state as a `Snapshot` without pushing it
    /// to either stack. `undo()` / `redo()` use this to remember the
    /// "other side" of a roll.
    fn capture_snapshot(&self) -> Snapshot {
        Snapshot {
            nodes: self.nodes.with_untracked(|m| m.clone()),
            edges: self.edges.with_untracked(|m| m.clone()),
            selection: self.selection.get_untracked(),
            next_node_id: self.next_node_id.get_untracked(),
            next_edge_id: self.next_edge_id.get_untracked(),
            is_dirty: self.is_dirty.get_untracked(),
        }
    }

    /// Swap the current graph state for the given snapshot. The reactive
    /// `.set()` calls fan out to every subscriber (canvas, sidebar, …).
    fn apply_snapshot(&self, snap: Snapshot) {
        self.nodes.set(snap.nodes);
        self.edges.set(snap.edges);
        self.selection.set(snap.selection);
        self.next_node_id.set(snap.next_node_id);
        self.next_edge_id.set(snap.next_edge_id);
        self.is_dirty.set(snap.is_dirty);
    }

    /// Push the current state onto the undo stack and clear redo. Called
    /// at the *start* of any mutation that should be undoable. While a
    /// transaction is open this is a no-op — `begin_transaction()`
    /// already took the one snapshot for the whole group.
    pub fn push_undo_snapshot(&self) {
        if self.transaction_depth.get_untracked() > 0 {
            return;
        }
        let snap = self.capture_snapshot();
        let mut stack = self.undo_stack.get_untracked();
        stack.push_back(snap);
        while stack.len() > MAX_UNDO_STACK {
            stack.pop_front();
        }
        self.undo_stack.set(stack);
        // Any new mutation invalidates the redo future.
        let mut redo = self.redo_stack.get_untracked();
        if !redo.is_empty() {
            redo.clear();
            self.redo_stack.set(redo);
        }
    }

    /// Group several mutations so the whole batch is a single undo step.
    /// Nest-safe via a depth counter — only the outermost `begin` takes
    /// a snapshot.
    ///
    /// Always pair with [`AppState::end_transaction`], or use the
    /// [`AppState::with_transaction`] closure wrapper.
    pub fn begin_transaction(&self) {
        let depth = self.transaction_depth.get_untracked();
        if depth == 0 {
            // Capture BEFORE any mutation happens inside the
            // transaction. Inner mutations see `depth > 0` and skip
            // their own snapshots.
            let snap = self.capture_snapshot();
            let mut stack = self.undo_stack.get_untracked();
            stack.push_back(snap);
            while stack.len() > MAX_UNDO_STACK {
                stack.pop_front();
            }
            self.undo_stack.set(stack);
            let mut redo = self.redo_stack.get_untracked();
            if !redo.is_empty() {
                redo.clear();
                self.redo_stack.set(redo);
            }
        }
        self.transaction_depth.set(depth + 1);
    }

    /// End a transaction started by [`AppState::begin_transaction`].
    pub fn end_transaction(&self) {
        self.transaction_depth
            .update(|d| *d = d.saturating_sub(1));
    }

    /// Convenience wrapper: run `f`, snapshotting once around the whole
    /// call. Anything `f` does to the graph is a single undo step.
    pub fn with_transaction<R>(&self, f: impl FnOnce() -> R) -> R {
        self.begin_transaction();
        let out = f();
        self.end_transaction();
        out
    }

    /// Roll back to the most recent pre-mutation state. Moves the current
    /// state onto the redo stack. Returns `true` iff something was
    /// actually undone.
    pub fn undo(&self) -> bool {
        let mut undo = self.undo_stack.get_untracked();
        let Some(snap) = undo.pop_back() else {
            return false;
        };
        let current = self.capture_snapshot();
        let mut redo = self.redo_stack.get_untracked();
        redo.push_back(current);
        while redo.len() > MAX_UNDO_STACK {
            redo.pop_front();
        }
        self.apply_snapshot(snap);
        self.undo_stack.set(undo);
        self.redo_stack.set(redo);
        true
    }

    /// Re-apply a state previously rolled away by [`AppState::undo`].
    /// Returns `true` iff something was actually redone.
    pub fn redo(&self) -> bool {
        let mut redo = self.redo_stack.get_untracked();
        let Some(snap) = redo.pop_back() else {
            return false;
        };
        let current = self.capture_snapshot();
        let mut undo = self.undo_stack.get_untracked();
        undo.push_back(current);
        while undo.len() > MAX_UNDO_STACK {
            undo.pop_front();
        }
        self.apply_snapshot(snap);
        self.undo_stack.set(undo);
        self.redo_stack.set(redo);
        true
    }

    // --------- graph mutations ---------

    pub fn add_node(&self, entity_type: EntityType, position: (f64, f64)) -> NodeId {
        self.push_undo_snapshot();
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
        self.mark_dirty();
        id
    }

    /// Like `add_node`, but with caller-supplied property values. Any
    /// property keys not in the entity's default schema are appended at
    /// the end; missing keys retain their default empty string. Does not
    /// change selection (unlike `add_node`, which picks the new node) —
    /// gadgets that spawn many children shouldn't scroll the sidebar.
    pub fn add_node_with_properties(
        &self,
        entity_type: EntityType,
        position: (f64, f64),
        overrides: &[(&str, &str)],
    ) -> NodeId {
        self.push_undo_snapshot();
        let id = NodeId(self.next_node_id.get_untracked());
        self.next_node_id.update(|n| *n += 1);
        let mut properties = entity_type.default_properties();
        for (k, v) in overrides {
            // IndexMap::insert overwrites existing keys and preserves
            // insertion order for new ones.
            properties.insert((*k).to_string(), (*v).to_string());
        }
        let node = Node {
            id,
            entity_type,
            position,
            properties,
        };
        self.nodes.update(|m| {
            m.insert(id, node);
        });
        self.mark_dirty();
        id
    }

    /// Update (or insert) a single property on a node and mark the graph
    /// dirty. Does NOT participate in undo — per-keystroke snapshots
    /// would balloon the undo stack. A future improvement is to
    /// debounce-snapshot on blur, tracked separately.
    pub fn update_node_property(&self, id: NodeId, key: &str, value: String) {
        self.nodes.update(|m| {
            if let Some(n) = m.get_mut(&id) {
                n.properties.insert(key.to_string(), value);
            }
        });
        self.mark_dirty();
    }

    /// Remove a node by id, also pruning every edge that touches it.
    /// No-op if the node doesn't exist. Returns `true` iff something was
    /// actually removed.
    pub fn remove_node(&self, id: NodeId) -> bool {
        let existed = self
            .nodes
            .with_untracked(|m| m.contains_key(&id));
        if !existed {
            return false;
        }
        self.push_undo_snapshot();
        self.nodes.update(|m| {
            m.shift_remove(&id);
        });
        // Drop dangling edges.
        self.edges.update(|m| {
            m.retain(|_, e| e.from != id && e.to != id);
        });
        // If the removed node was selected, clear the selection.
        if self.selection.get_untracked() == Selection::Node(id) {
            self.selection.set(Selection::None);
        }
        self.mark_dirty();
        true
    }

    /// Remove a single edge by id. No-op if it doesn't exist. Clears
    /// selection if the removed edge was selected.
    pub fn remove_edge(&self, id: EdgeId) -> bool {
        let existed = self
            .edges
            .with_untracked(|m| m.contains_key(&id));
        if !existed {
            return false;
        }
        self.push_undo_snapshot();
        self.edges.update(|m| {
            m.shift_remove(&id);
        });
        if self.selection.get_untracked() == Selection::Edge(id) {
            self.selection.set(Selection::None);
        }
        self.mark_dirty();
        true
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
        // Snapshot AFTER early-returns so no-op calls don't pollute the
        // undo stack with identical-looking entries.
        self.push_undo_snapshot();
        let id = EdgeId(self.next_edge_id.get_untracked());
        self.next_edge_id.update(|n| *n += 1);
        self.edges.update(|m| {
            m.insert(id, Edge { id, from, to, label: None });
        });
        self.mark_dirty();
        Some(id)
    }

    pub fn clear(&self) {
        self.nodes.update(|n| n.clear());
        self.edges.update(|e| e.clear());
        self.selection.set(Selection::None);
        self.viewport.set(Viewport::default());
        self.drag.set(DragKind::None);
        self.current_file_path.set(None);
        // Fresh empty graph matches "nothing on disk" — start clean.
        self.mark_clean();
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
        // Freshly loaded from disk — the in-memory state matches the
        // file byte-for-byte.
        self.mark_clean();
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

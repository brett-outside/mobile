use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an event in the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,         // Unique identifier (UUID or hash)
    pub timestamp: DateTime<Utc>, // UTC timestamp ensuring ordering
    pub origin_id: String,        // Unique device identifier
    pub event_type: String,       // Type of event (join, leave, data update)
    pub payload: serde_json::Value, // Event-specific data as JSON
    pub signature: String,        // Digital signature for integrity verification
}

/// Represents a node (mobile instance) in the network
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub node_id: String,          // Unique identifier for the node
    pub last_active: DateTime<Utc>, // Last active timestamp
    pub status: NodeStatus,       // Current status of the node
}

/// Enum for node status
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Active,
    Inactive,
}

/// Event log that maintains authenticated events
pub struct EventLog {
    events: Arc<Mutex<Vec<Event>>>,
}

impl EventLog {
    pub fn new() -> Self {
        EventLog {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a new event to the log after verifying its signature
    pub fn add_event(&self, event: Event) -> Result<(), String> {
        if !self.verify_signature(&event) {
            return Err("Invalid event signature".to_string());
        }
        
        let mut events = self.events.lock().unwrap();
        events.push(event);
        Ok(())
    }

    /// Retrieve events since a specific timestamp
    pub fn get_events_since(&self, timestamp: DateTime<Utc>) -> Vec<Event> {
        let events = self.events.lock().unwrap();
        events.iter()
            .filter(|e| e.timestamp > timestamp)
            .cloned()
            .collect()
    }

    /// Verify the digital signature of an event
    fn verify_signature(&self, event: &Event) -> bool {
        // Placeholder for actual signature verification logic
        // In a real implementation, this would use cryptographic libraries
        true
    }
}

/// Registry for tracking nodes in the network
pub struct NodeRegistry {
    nodes: Arc<Mutex<HashMap<String, Node>>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        NodeRegistry {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register or update a node
    pub fn register_node(&self, node: Node) {
        let mut nodes = self.nodes.lock().unwrap();
        nodes.insert(node.node_id.clone(), node);
    }

    /// Get a list of all active nodes
    pub fn get_active_nodes(&self) -> Vec<Node> {
        let nodes = self.nodes.lock().unwrap();
        nodes.values()
            .filter(|n| n.status == NodeStatus::Active)
            .cloned()
            .collect()
    }

    /// Mark a node as inactive
    pub fn mark_node_inactive(&self, node_id: &str) {
        let mut nodes = self.nodes.lock().unwrap();
        if let Some(node) = nodes.get_mut(node_id) {
            node.status = NodeStatus::Inactive;
        }
    }
}

/// The main server structure that manages the event log and node registry
pub struct MasterServer {
    event_log: EventLog,
    node_registry: NodeRegistry,
}

impl MasterServer {
    pub fn new() -> Self {
        MasterServer {
            event_log: EventLog::new(),
            node_registry: NodeRegistry::new(),
        }
    }

    /// API endpoint: Log a new event
    pub fn log_event(&self, event: Event) -> Result<(), String> {
        self.event_log.add_event(event)
    }

    /// API endpoint: Retrieve events since a specific timestamp
    pub fn get_events_since(&self, timestamp: DateTime<Utc>) -> Vec<Event> {
        self.event_log.get_events_since(timestamp)
    }

    /// API endpoint: Register a node
    pub fn register_node(&self, node: Node) {
        self.node_registry.register_node(node)
    }

    /// API endpoint: Get a list of active nodes
    pub fn get_active_nodes(&self) -> Vec<Node> {
        self.node_registry.get_active_nodes()
    }
}
# Mobile Prototype

## Overview
This is a specification designed for a secure mobile app that displays a dynamic network graph in real time. This specification covers the high-level architecture, key components, communication protocols, security measures, and implementation details. Although the design accommodates a fully distributed peer-to-peer network, it also includes a master server (written in Rust) to serve as a backup and recovery mechanism for the mobile instances (written in Kotlin).
 
1. Overview
Purpose:
Create a mobile app that not only visualizes a dynamic network graph in real time but also enables every instance to broadcast and receive events securely without relying on a central server—unless necessary for backup and data recovery. This architecture supports resilience, scalability, and fault tolerance.
Key Components:
    - Mobile Front End (Android in Kotlin): Handles the dynamic user interface, real-time graph visualization, and direct peer-to-peer (P2P) communications.
    - Master Server (Rust Back End): Acts as the central backup for event synchronization, persistent state storage, and recovery support in case of connectivity or peer failure.  
 
2. Architecture Details
   
   2.1 Distributed Network Mode (Peer-to-Peer)
    - Peer Discovery:
Mobile instances use a discovery mechanism (e.g., multicast DNS or a lightweight rendezvous protocol) to identify available peers on the network. Each instance maintains a list of active peers and periodically sends heartbeat messages to ensure connectivity.
    - Event Propagation:
When an instance generates an event, it broadcasts the event directly to all identified peers. The network graph updates dynamically as events are processed. The protocol should support:
        - Event types: Join/leave notifications, state changes, and custom application events.
        - Event structure: JSON or Protobuf payloads containing a unique event identifier, timestamp, origin identifier, event type, and event data.
    - Resilience:
If some peers become unreachable or lose synchronization, instances are designed to seamlessly switch roles—either by re-establishing P2P connectivity or by querying the backup master server.

   2.2 Master Server Backup
    - Backup Role:
The master server is not involved in primary event routing during normal P2P operation. Instead, it keeps an authenticated log of events that can be used for recovery or synchronizing a node that reconnects to the network.
    - Recovery Procedures:
        - On rejoining, an instance can query the master server to fetch missed or historic events.
        - The master server also serves as a coordination point for new nodes that are unable to immediately discover peers.
    - Scalability & Redundancy:
The backup server is optimized for state persistence and quick lookup rather than high-frequency event propagation, reducing performance overhead on the distributed network.
 
3. Mobile App (Android/Kotlin) Specification
   
   3.1 User Interface and Visualization
    - Dynamic Network Graph:
        - Display nodes representing mobile instances and edges representing real-time communications.
        - Use an efficient graph rendering library (e.g., a custom OpenGL view, Jetpack Compose with Canvas APIs, or third-party libraries optimized for dynamic graphs).
        - Support smooth animations to reflect rapid network changes.
    - Event Interaction:
        - Users can tap/click on nodes to display additional metadata (e.g., unique identifier, last active timestamp, event history).
        - Real-time notifications and updates appear as overlays or integrated into the UI.
   3.2 Communication and Event Handling
    - Event Generation:
        - APIs to create events including schema validation, ensuring each event has fields like:
| Field Name | Type | Description | | ------------ | ------- | ------------------------------------------ | | event_id | String | Unique identifier (UUID or hash) | | timestamp | DateTime| UTC timestamp ensuring ordering | | origin_id | String | Unique device identifier | | event_type | String | Type of event (join, leave, data update) | | payload | JSON | Event-specific data |
    - Event Broadcasting & Reception:
        - Implement a messaging layer (using secure sockets or a library supporting P2P communications) where each event is encrypted and signed.
        - Event listeners update the network graph in real time upon reception.
        - The app should include mechanisms for de-duplicating events and handling out-of-order reception.
   3.3 Local Security Measures
    - Secure Storage:
        - Sensitive information (e.g., cryptographic keys) is stored in Android’s secure Keystore.
    - Communication Security:
        - End-to-end encryption (e.g., TLS or DTLS in a P2P scenario).
        - Mutual authentication using client certificates or public-key cryptography.
        - Message signing to ensure integrity and authenticity.
   3.4 Offline and Failover Behaviour
    - Offline Operations:
        - The app should cache recent events and network topology locally.
        - On resuming connectivity, local changes are synchronized with peers and the master server.
    - Failover Modes:
        - Automatic detection of failed peers and re-establishing P2P links.
        - Fallback to master server synchronization if the peer network quorum falls below a threshold.
 
4. Master Server (Rust Back End) Specification
   4.1 Core Functions and API
    - API Endpoints:
        - Event Log Endpoint:
POST /event to log new events with digital signatures verification.
GET /events?since=<timestamp> to retrieve missed events.
        - Node Registry and Discovery:
POST /node when a mobile instance registers or re-registers with the backup system. GET /nodes to list currently known nodes (if network state summary is allowed).
    - Data Schema:
        - Persist events with fields similar to the mobile event schema.
        - Maintain an index of nodes with their current status (active, inactive).
   4.2 Security and Reliability
    - Secure Communication:
        - All endpoints are exposed over HTTPS/TLS.
        - Strict certificate pinning and transport security measures are enforced.
    - Data Integrity and Auditing:
        - Each event is validated against its signature before being stored.
        - Maintain an immutable log for audit and rollback—supporting eventual consistency with distributed instances.
    - Resilience:
        - High availability through database replication or clustering.
        - Data backup routines and monitoring for unusual patterns to pre-empt possible attacks.
 
5. Communication Protocol and Data Flow
   
   5.1 Communication Protocol
    - Peer-to-Peer Layer:
        - Use a well-defined protocol (custom lightweight protocol layered on UDP/TCP or leveraging libraries like libp2p) for exchanging events.
        - Ensure each message has metadata for security (e.g., nonce, timestamp, digital signature).
    - Master Server Fallback:
        - Minimal API calls: only used if a node detects a discrepancy in the event log or experiences connectivity issues with peers.
        - The protocol supports reconciliation by comparing sequence numbers or timestamps.
   5.2 Data Flow Diagram (ASCII Representation)
  
6. Security Considerations
   
   6.1 Secure Messaging
    - Encryption:
All message payloads are encrypted end-to-end, using libraries and protocols that mitigate MITM (Man-in-the-Middle) attacks.
    - Authentication:
Each instance must validate peers through mutual certificate exchange or pre-shared public keys.
    - Integrity Checks:
Use digital signatures and hashed message authentication codes (HMACs) to verify that messages have not been tampered with.
    - Replay Protection:
Incorporate nonces and timestamp validations to prevent replay attacks.
   6.2 Local Device Security
    - Secure Storage:
Sensitive cryptographic material is stored in Android’s hardware-backed Keystore.
    - Sandboxing:
Application data and caches are segregated to prevent leakage between different apps or processes.
    - Regular Security Audits:
Both the mobile app and master server code undergo periodic security reviews and penetration testing.
 
7. Implementation Roadmap

   7.1 Mobile Front End (Kotlin)
    - Prototype UI:
Initial designs for dynamic network graph visualization using Jetpack Compose or similar UI frameworks.
    - Integration of P2P Messaging:
Develop the communication layer with secure channels.
    - Local caching and offline support:
Implement local databases to store temporary events.
    - Security Testing:
Rigorous testing of encryption, authentication, and secure storage techniques.
7.2 Master Server (Rust)
    - API Development:
Build RESTful endpoints using frameworks like Actix Web or Rocket.
    - Event Log Management:
Develop a secure, immutable event store, potentially with a lightweight embedded database.
    - Scalability Tests:
Validate the architecture under simulated network conditions.
    - Security Hardening:
Integrate stringent TLS configurations and conduct security audits.
 
8. Summary and Further Considerations
    
This specification outlines a hybrid, secure, and real-time mobile application architecture supporting dynamic visualization of network graphs with a distributed P2P backbone complemented by a master server for backup and recovery. Key innovation points include the tight integration of:
- Distributed peer-to-peer event propagation, ensuring real-time responsiveness and resilience.
- Robust cryptographic mechanisms, ensuring end-to-end security for communications.
- Seamless failover and synchronization procedures, enabling sustained network performance even under partial failure conditions.
    
Additional Thoughts:
- Monitoring & Diagnostics: Incorporate real-time monitoring dashboards (both on mobile and server side) for network health and anomaly detection.
- Extensibility: Consider future extensions such as supporting different types of graphs or integrating collaboration features where the network graph represents not just connectivity but shared data or media streams.
- User Privacy: In addition to technical measures, ensure that data handling complies with data protection regulations (e.g., GDPR) and that user consent is clearly managed.

This document explains how to handle complex data types in Kinode message passing. Key points for an LLM:

1. Data Serialization:
   - Using Serde for serialization/deserialization
   - Options like bincode, MessagePack, serde_json
   - WIT language for type definitions
   - Language-independent API definitions

2. Message Types:
   - Custom enum/struct definitions in WIT
   - Generated Rust types from WIT
   - Benefits of structured message types
   - Type safety and documentation
   - Interoperability between processes

3. Message Handling:
   - Parsing complex message types
   - Request/Response pattern with enums
   - Error handling for invalid messages
   - Capability management for inter-process communication

4. Process Lifecycle:
   - on_exit configurations: None, Restart, JSON object
   - Process crash handling
   - Restart behavior
   - Exit notifications

Relevant for tasks involving:
- Complex data type handling
- API definition and documentation
- Process lifecycle management
- Inter-process communication
- Error handling
- Type-safe messaging
- Process resilience patterns
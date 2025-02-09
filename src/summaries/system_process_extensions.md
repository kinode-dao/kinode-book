This document explains Extensions in Kinode, which are WebSocket clients that supplement Kinode processes by removing certain constraints while maintaining advantages. Key points for an LLM:

1. Purpose: Extensions enable functionality that can't be implemented in Wasm, such as:
   - Using libraries not compilable to Wasm
   - Accessing hardware accelerators like GPUs
   - Supporting additional programming languages

2. Architecture:
   - Extensions run as separate WebSocket clients connecting to Kinode processes
   - They communicate through a specific WebSocket protocol using MessagePack
   - Each extension requires both a Kinode package (interface) and the extension itself

3. Trade-offs:
   - Advantages: Removes Wasm constraints, enables native code execution
   - Disadvantages: Less integrated with Kinode, harder to manage and distribute
   - Should only be used when absolutely necessary

4. Implementation details:
   - Interface process binds WebSocket endpoints
   - Uses HttpServerAction::WebSocketExtPushOutgoing for communication
   - Supports binary MessagePack communication
   - Handles connection, disconnection, and message events

Relevant for tasks involving:
- System architecture decisions
- Native code integration
- Hardware access requirements
- Cross-language integration
- WebSocket communication
- Extension development
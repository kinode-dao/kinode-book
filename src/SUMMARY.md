# Summary

- Getting Started
  - [Quick Start](./getting_started/quick_start.md)
    This document provides a practical, hands-on introduction to Kinode through a chat application example. Relevant for tasks involving:
    - Setting up the Kinode development environment
    - Running multiple test nodes
    - Creating and building a basic Kinode application
    - Understanding package deployment and inter-node communication
    - Basic message injection and testing
    - Learning kit command-line tool usage

    Key terms: kit, fake nodes, package deployment, message injection, chat application, development setup
  - [Introduction](./getting_started/intro.md)
    This document provides an introduction to the Kinode operating system. Key points for an LLM:

    1. Core Concepts:
       - Decentralized operating system
       - Peer-to-peer app framework
       - Node network
       - Sovereign cloud computer
       - Personal server control

    2. Fundamental Primitives:
       - Networking: Peer-to-peer message passing
       - Identity: Permanent system-wide identities
       - Data Persistence: Permanent data storage
       - Global State: Blockchain interaction

    3. Architecture Components:
       - Process-based applications
       - Wasm-compiled programs
       - Kernel message handling
       - KNS (Kinode Name System)
       - Ethereum integration
       - Remote backup system

    4. Built-in Features:
       - HTTP server/client framework
       - Key-value store
       - File system abstraction
       - Chain read/write capabilities
       - Application deployment system

    Relevant for tasks involving:
    - System architecture understanding
    - P2P application development
    - Blockchain integration
    - Data persistence
    - Network identity management
    - Process communication
    - Application deployment
    - System Components
  - Processes
    - [Process Semantics](./system/process/processes.md)
      This document provides comprehensive details about Kinode's process system. Relevant for tasks involving:
      - Understanding process architecture and identification
      - Working with process state management
      - Implementing request/response patterns
      - Managing process communication and message passing
      - Working with lazy loading and blob inheritance
      - Error handling in process communication
      - Child process management and spawning
      - Understanding process security and capabilities
      - Working with Wasm components in Kinode

      Key terms: processes, PackageId, ProcessId, Address, state management, request/response, message passing, child processes, Wasm components, capabilities
    - [Capability-Based Security](./system/process/capabilities.md)
      This document explains Kinode's capability-based security system. Relevant for tasks involving:
      - Implementing security in Kinode applications
      - Managing process permissions and access control
      - Working with capability tokens and their verification
      - Setting up manifest.json for process capabilities
      - Creating and managing custom capabilities
      - Implementing capability-based authentication
      - Understanding system vs userspace capabilities

      Key terms: capabilities, security tokens, manifest.json, process permissions, authentication, networking capabilities, capability granting
    - [Startup, Spindown, and Crashes](./system/process/startup.md)
      This document explains process lifecycle management in Kinode. Relevant for tasks involving:
      - Managing process initialization and shutdown
      - Configuring process exit behaviors (None, Restart, Requests)
      - Handling process crashes and recovery
      - Managing parent-child process relationships
      - Implementing state persistence across restarts
      - Understanding kernel process management
      - Working with process capabilities during exit
      - Best practices for process lifecycle management

      Key terms: process initialization, OnExit behaviors, crash handling, state persistence, parent-child processes, process restarts, exit handling, lifecycle management
    - [WIT APIs](./system/process/wit_apis.md)
      This document explains how Kinode uses WebAssembly Interface Types (WIT) for APIs. Relevant for tasks involving:
      - Understanding WebAssembly components in Kinode
      - Creating and managing process APIs
      - Working with WIT language and conventions
      - Defining types, interfaces, and worlds in WIT
      - Managing package dependencies through WIT
      - Compiling WIT to different target languages
      - Implementing cross-language API interfaces
      - Understanding API versioning and packaging

      Key terms: WIT, WebAssembly components, interfaces, worlds, API definitions, type systems, package dependencies, cross-language compatibility
- Kit: Development Tool**kit**
  - [`boot-fake-node`](./kit/boot-fake-node.md)
    This document explains how to work with fake Kinode nodes for development. Relevant for tasks involving:
    - Setting up development environments
    - Creating test networks of nodes
    - Managing local node communication
    - Working with test blockchains
    - Testing inter-node messaging
    - Managing node state persistence
    - Configuring development ports
    - Setting up isolated testing environments

    Key terms: fake nodes, development environment, test networks, anvil, local testing, node communication, state persistence, development configuration
  - [`new`](./kit/new.md)
    This document explains how to create new Kinode packages using the kit tool. Relevant for tasks involving:
    - Creating new Kinode package templates
    - Understanding available package templates and their purposes
    - Working with UI-enabled vs non-UI packages
    - Setting up different types of example applications
    - Understanding package naming conventions
    - Configuring package publishers and languages
    - Getting started with Kinode development

    Key terms: kit new, templates, package creation, UI templates, Kimap-safe names, publisher configuration, development setup
  - [`build`](./kit/build.md)
    This document explains how to build Kinode packages using kit. Relevant for tasks involving:
    - Building Kinode packages from source code
    - Managing multi-process package builds
    - Building UI components
    - Managing build dependencies
    - Configuring build options and features
    - Working with different programming languages
    - Creating reproducible builds
    - Managing build artifacts and outputs

    Key terms: kit build, package building, Wasm compilation, UI building, build configuration, dependencies, reproducible builds, build artifacts
  - [`start-package`](./kit/start-package.md)
    This document explains how to deploy packages to Kinode nodes. Relevant for tasks involving:
    - Installing packages on Kinode nodes
    - Starting built packages
    - Managing package deployment
    - Working with package metadata
    - Managing node connections for deployment
    - Understanding deployment workflow
    - Package initialization and startup
    - Managing package binaries

    Key terms: package deployment, package installation, node connections, package metadata, deployment workflow, package initialization
  - [`publish`](./kit/publish.md)
    This document explains how to publish packages to the Kinode network. Relevant for tasks involving:
    - Publishing packages to Kimap
    - Managing package metadata and URIs
    - Working with blockchain transactions
    - Managing package updates and unpublishing
    - Configuring gas and transaction fees
    - Working with hardware wallets
    - Managing package deployment
    - Publishing to test vs real networks

    Key terms: package publishing, Kimap, metadata, blockchain deployment, gas configuration, transaction management, hardware wallets, network deployment
  - [`build-start-package`](./kit/build-start-package.md)
    This document explains how to build and deploy packages in one step. Relevant for tasks involving:
    - Combined package building and deployment
    - Streamlined development workflows
    - Managing build and deployment options
    - Working with package dependencies
    - Configuring build features
    - Managing UI components
    - Handling reproducible builds
    - Development automation

    Key terms: package building, package deployment, build configuration, dependency management, reproducible builds, development workflow, automation
  - [`remove-package`](./kit/remove-package.md)
    This document explains how to remove packages from Kinode nodes. Relevant for tasks involving:
    - Uninstalling packages from nodes
    - Managing package installations
    - Working with package IDs
    - Managing node package state
    - Package cleanup operations
    - Understanding package identification
    - Managing package publishers
    - Node package management

    Key terms: package removal, uninstallation, package identification, package management, publisher management, node operations, cleanup
  - [`chain`](./kit/chain.md)
    This document explains how to work with local blockchain networks in Kinode development. Relevant for tasks involving:
    - Setting up local blockchain networks
    - Working with Anvil for development
    - Managing blockchain state
    - Testing smart contract interactions
    - Working with KNS and app-store contracts
    - Managing chain configurations
    - Testing blockchain events
    - Setting up development chains

    Key terms: anvil, blockchain, local chain, KNS, app-store, chain state, smart contracts, development environment
  - [`dev-ui`](./kit/dev-ui.md)
    This document explains how to develop UIs for Kinode packages. Relevant for tasks involving:
    - Setting up UI development environments
    - Working with hot reloading for UI changes
    - Managing UI development servers
    - Configuring UI development ports
    - Managing UI dependencies
    - Creating production UI builds
    - UI development workflow optimization

    Key terms: UI development, hot reloading, development server, UI dependencies, production builds, web development
  - [`inject-message`](./kit/inject-message.md)
    This document explains how to inject messages into Kinode nodes. Relevant for tasks involving:
    - Testing process communication
    - Injecting data into nodes
    - Scripting node interactions
    - Managing message responses
    - Working with file data injection
    - Testing process functionality
    - Automating node interactions
    - Managing blocking vs non-blocking messages

    Key terms: message injection, process communication, data injection, scripting, message responses, node interaction, testing automation
  - [`run-tests`](./kit/run-tests.md)
    This document explains how to run tests in Kinode. Relevant for tasks involving:
    - Running package tests
    - Setting up test environments
    - Working with test configuration files
    - Managing test packages and dependencies
    - Using the tester package
    - Testing with multiple fake nodes
    - Understanding test orchestration
    - Managing test pass/fail conditions

    Key terms: testing, test packages, test environments, fake nodes, test configuration, tester package, test orchestration
  - [`connect`](./kit/connect.md)
    This document explains how to manage SSH connections to remote Kinode nodes. Relevant for tasks involving:
    - Creating SSH tunnels to remote nodes
    - Managing remote development connections
    - Setting up secure node access
    - Working with remote node ports
    - Managing SSH authentication
    - Configuring development tooling
    - Managing tunnel connections and disconnections
    - Remote development workflow

    Key terms: SSH tunnels, remote nodes, authentication, port forwarding, remote development, secure connections, SSH configuration
  - [`boot-real-node`](./kit/boot-real-node.md)
    This document explains how to start real Kinode nodes. Relevant for tasks involving:
    - Starting production Kinode instances
    - Managing node versions and binaries
    - Configuring node home directories
    - Setting up network ports and RPC endpoints
    - Managing node verbosity and logging
    - Working with prebuilt vs local binaries
    - Setting up new nodes vs booting existing ones
    - Managing live network connections

    Key terms: real nodes, node configuration, network setup, binary management, node versioning, RPC configuration, production deployment
- My First Kinode Application
  - [Environment Setup](./my_first_app/chapter_1.md)
    This document provides a comprehensive guide to setting up a Kinode development environment. Key points for an LLM:

    1. Development Setup:
       - Installing Rust and Kinode Development Tools (kit)
       - Creating new packages with kit new
       - Package structure and organization
       - Directory layout and key files
       - Environment requirements (Unix/WSL)

    2. Package Components:
       - manifest.json: Process configuration and capabilities
       - metadata.json: Package metadata and versioning
       - API directory: Public interface definitions
       - Process directories: Source code organization
       - Test directory: Package testing setup

    3. Development Workflow:
       - Building packages with kit build
       - Starting fake nodes for development
       - Package deployment with kit start-package
       - Inter-node communication testing
       - Running package tests
       - Message injection and testing

    4. Configuration Details:
       - Process naming rules
       - Package dependencies
       - WIT version management
       - Capability configuration
       - Network settings
       - Port configuration

    Relevant for tasks involving:
    - Development environment setup
    - Package creation and structure
    - Process configuration
    - Testing setup
    - Node deployment
    - Inter-process communication
    - Package metadata management
    - Development workflow setup
  - [Sending and Responding to a Message](./my_first_app/chapter_2.md)
    This document explains how to implement basic message sending and receiving in a Kinode process. Key points for an LLM:

    1. Core Concepts:
       - WIT Bindings for Wasm component interface
       - Process initialization with init() function
       - Request-Response messaging pattern
       - Message handling loops

    2. Technical Components:
       - WIT bindings generation using wit_bindgen
       - Component struct implementation
       - Process initialization using call_init! macro
       - Message handling with await_message()

    3. Message Handling:
       - Creating and sending Requests
       - Setting response expectations and timeouts
       - Handling incoming Messages
       - Sending Responses
       - Error handling for timeouts and network issues

    4. Code Structure:
       - Basic boilerplate requirements
       - Message loop pattern
       - Script-like pattern
       - Error handling patterns

    Relevant for tasks involving:
    - Basic Kinode process development
    - Message passing implementation
    - Process initialization
    - Request-Response patterns
    - Process loops
    - Wasm component interfaces
  - [Messaging with More Complex Data Types](./my_first_app/chapter_3.md)
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
  - [Frontend Time](./my_first_app/chapter_4.md)
    This document explains how to add HTTP functionality and a frontend to a Kinode process. Key points for an LLM:

    1. HTTP Request Handling:
       - Using kinode_process_lib for HTTP functionality
       - Path binding with HttpServer
       - Authentication configuration
       - Request/Response handling patterns
       - HTTP method discrimination (GET/PUT)

    2. Frontend Integration:
       - Static file serving
       - UI directory structure
       - HTML file serving
       - API endpoint setup
       - Authentication modes for different endpoints
       - Frontend-backend communication

    3. Homepage Integration:
       - Adding app icons
       - Base64 image encoding
       - Homepage button configuration
       - Widget implementation
       - Widget HTML/iframe setup
       - App docking functionality

    4. Implementation Details:
       - Capability management for HTTP
       - Request type discrimination
       - Static vs dynamic content serving
       - Frontend build integration
       - Widget case studies

    Relevant for tasks involving:
    - Frontend development
    - HTTP endpoint setup
    - UI integration
    - Homepage customization
    - Widget development
    - Authentication configuration
    - Static file serving
    - Process-frontend communication
  - [Sharing with the World](./my_first_app/chapter_5.md)
    This document explains how to share and publish a Kinode process package. Key points for an LLM:

    1. Package Preparation:
       - Adding repository URL to metadata.json
       - Using permanent commit links instead of branch URLs
       - Populating code_hashes field
       - Reviewing manifest.json and metadata.json
       - Package and publisher naming

    2. Publishing Methods:
       - Using App Store GUI interface
       - Using kit publish command line tool
       - Installing on real node vs fake node
       - Publishing to the network

    3. Authentication Requirements:
       - Keystore creation and usage
       - Hardware wallet options (Ledger/Trezor)
       - ETH RPC endpoint setup
       - Alchemy API key requirements

    4. Publishing Process:
       - Command line syntax
       - Required parameters
       - Keystore usage
       - Network verification

    Relevant for tasks involving:
    - Package distribution
    - App publishing
    - Authentication setup
    - Network deployment
    - Package metadata management
    - Repository linking
    - Key management
    - RPC configuration
- Cookbook (Handy Recipes)
  - [Saving State](./cookbook/save_state.md)
    This document explains how to manage persistent state in Kinode processes. Relevant for tasks involving:
    - Using set_state and get_state system calls
    - Implementing process state persistence
    - Managing state between process restarts
    - Handling state serialization and deserialization
    - Working with state format migrations
    - Choosing between state storage options
    - Performance considerations for state operations
    - Implementing stateful processes

    Key terms: state persistence, set_state, get_state, serialization, state management, process restarts, state migration, storage options
  - [Managing Child Processes](./cookbook/manage_child_processes.md)
    This document explains how to work with child processes in Kinode. Relevant for tasks involving:
    - Creating and managing child processes
    - Setting up parent-child process relationships
    - Configuring process spawn parameters
    - Managing process capabilities inheritance
    - Handling process exit behaviors
    - Organizing multi-process package structures
    - Implementing worker processes
    - Managing process naming and identification

    Key terms: child processes, spawn function, parent process, process capabilities, OnExit behavior, worker processes, process management, Wasm paths
  - [Publishing a Website or Web App](./cookbook/publish_to_web.md)
    This document explains how to serve web content from Kinode. Relevant for tasks involving:
    - Setting up web servers in Kinode
    - Serving static web content
    - Managing web assets and files
    - Configuring HTTP servers
    - Managing content caching
    - Setting up authentication
    - Managing content security
    - Optimizing web content delivery

    Key terms: web server, static content, HTTP server, asset management, content caching, web security, content delivery, server configuration
  - [Simple File Transfer Guide](./cookbook/file_transfer.md)
    This document provides a detailed walkthrough of implementing file transfer functionality in Kinode. Relevant for tasks involving:
    - Building applications that need to transfer files between nodes
    - Implementing manager-worker patterns for concurrent operations
    - Using the Virtual File System (VFS) for file storage
    - Creating and using WIT APIs for package interoperability
    - Understanding process spawning and inter-process communication
    - Implementing chunked file transfer protocols

    Key terms: file transfer, VFS, worker pattern, WIT API, process spawning, concurrent operations, chunks
  - [Intro to Web UI with File Transfer](./cookbook/file_transfer_ui.md)
    This document provides a comprehensive guide on building a UI for the file transfer app on Kinode. Key points for an LLM:

    1. Project Setup:
       - Uses kit's UI template with --ui flag
       - Integrates Vite/React for frontend development
       - Includes WebSocket communication for real-time updates

    2. Main Features:
       - File upload to node's VFS
       - File listing from local node
       - File search across other nodes
       - File download with progress tracking
       - Real-time progress updates via WebSocket

    3. Technical Implementation:
       - Uses React for UI framework
       - Zustand for state management
       - Tailwind CSS for styling
       - WebSocket integration for real-time updates
       - Vite configuration for development

    4. Components:
       - App.tsx: Main application component
       - MyFiles: Shows local node files
       - FileEntry: Individual file display/controls
       - SearchFiles: Network file search interface

    Relevant for tasks involving:
    - Frontend development on Kinode
    - File transfer UI implementation
    - WebSocket integration
    - React/Vite setup
    - Real-time progress tracking
    - Cross-node file operations
  - [Writing and Running Scripts](./cookbook/writing_scripts.md)
    This document explains how to create and use scripts in Kinode. Relevant for tasks involving:
    - Creating terminating processes (scripts) vs long-running applications
    - Implementing script initialization and argument handling
    - Publishing scripts through scripts.json configuration
    - Managing script capabilities and permissions
    - Using the script! macro for process creation
    - Calling scripts from the terminal
    - Creating aliases for frequently used scripts
    - Understanding script vs application differences

    Key terms: scripts, script! macro, scripts.json, process arguments, script capabilities, terminal commands, aliases, process termination
  - [Reading Data from ETH](./cookbook/reading_data_from_eth.md)
    This document explains how to read blockchain data in Kinode applications. Relevant for tasks involving:
    - Setting up Ethereum providers and connections
    - Reading blockchain data and state
    - Managing event subscriptions and filters
    - Handling blockchain data pagination
    - Working with smart contract events
    - Error handling for RPC calls
    - Managing provider timeouts and chain IDs
    - Implementing efficient log fetching strategies

    Key terms: Ethereum, RPC, provider, blockchain events, filters, subscriptions, eth:distro:sys, block number, smart contracts
  - [Writing Data to ETH](./cookbook/writing_data_to_eth.md)
    This document provides a detailed guide for writing data to Ethereum from Kinode. Relevant for tasks involving:
    - Creating and deploying smart contracts
    - Interacting with blockchain contracts from Kinode
    - Managing blockchain transactions and nonces
    - Working with Solidity and Rust integration
    - Implementing contract ABIs in Rust
    - Managing wallet and transaction signing
    - Testing with local blockchain networks
    - Error handling in blockchain interactions

    Key terms: smart contracts, Solidity, blockchain transactions, ABIs, wallet management, nonce handling, contract deployment, transaction signing
  - [Creating and Using Capabilities](./cookbook/creating_and_using_capabilities.md)
    This document explains how to implement custom capabilities in Kinode applications. Relevant for tasks involving:
    - Creating custom capability systems
    - Managing process permissions
    - Implementing capability checks
    - Working with manifest capabilities
    - Using capabilities in message passing
    - Defining capability types in WIT
    - Testing capability systems
    - Managing authorization flows

    Key terms: capabilities, permissions, manifest configuration, message authorization, capability validation, WIT definitions, authorization patterns, kernel security
  - [Managing Contacts](./cookbook/managing_contacts.md)
    This document explains how to work with Kinode's contacts system. Relevant for tasks involving:
    - Using the built-in contacts system
    - Managing contact capabilities
    - Implementing contact operations (read/write/modify)
    - Working with contact data structures
    - Managing contact permissions
    - Integrating contacts with applications
    - Contact system API usage
    - Contact data management

    Key terms: contacts system, contact capabilities, contact operations, contacts:contacts:sys, WIT APIs, contact management, permissions, contact data
  - [Talking to the Outside World](./cookbook/talking_to_the_outside_world.md)
    This document explains how Kinode communicates with external systems. Relevant for tasks involving:
    - Implementing HTTP clients and servers
    - Working with WebSocket connections
    - Managing external communications
    - Implementing request/response patterns
    - Setting up bidirectional communication
    - Managing network protocols
    - Working with external APIs
    - Implementing custom communication patterns

    Key terms: HTTP, WebSockets, external communication, network protocols, client/server implementation, request/response, bidirectional communication, API integration
  - [Exporting & Importing Package APIs](./cookbook/package_apis.md)
    This document explains how to work with Kinode package APIs. Relevant for tasks involving:
    - Creating and exporting package APIs using WIT
    - Importing and using APIs from other packages
    - Understanding API dependencies and their resolution
    - Working with WIT interfaces and worlds
    - Implementing API functions and types
    - Managing package dependencies in metadata.json
    - Building and testing API implementations
    - Remote file storage example implementation

    Key terms: WIT, API exports/imports, package dependencies, metadata.json, interface definitions, API functions, remote file storage
  - [Exporting Workers in Package APIs](./cookbook/package_apis_workers.md)
    This document explains how to work with workers in Kinode package APIs. Relevant for tasks involving:
    - Creating and exporting worker processes
    - Integrating workers with package APIs
    - Managing worker spawn operations
    - Implementing worker-based file transfers
    - Working with dependencies in metadata.json
    - Building and deploying worker-enabled packages
    - Combining chat and file transfer functionality
    - Managing worker lifecycle and communication

    Key terms: workers, package APIs, spawn operations, file transfer, worker processes, package dependencies, chat applications, process communication
- [Hosted Nodes User Guide](./hosted-nodes.md)
  This document provides guidance on using hosted Kinode instances. Relevant for tasks involving:
  - Managing hosted Kinode instances
  - Setting up SSH access to remote nodes
  - Configuring SSH keys and security
  - Creating SSH tunnels for development
  - Using kit with remote nodes
  - Managing HTTP RPC access
  - Working with Valet management interface
  - Setting up development environments

  Key terms: hosted nodes, SSH, SSH tunnels, Valet, remote development, kit configuration, HTTP RPC, security configuration
- [Glossary](./glossary.md)
  This document provides comprehensive definitions of Kinode-specific terminology. Relevant for tasks involving:
  - Understanding Kinode's technical vocabulary and concepts
  - Learning about core system components and their relationships
  - Understanding messaging and process architecture
  - Working with capabilities and security concepts
  - Distinguishing between local and remote operations
  - Understanding Wasm components and WIT interfaces
  - Working with Kinode's package system

  Key terms: process, message, capability, kernel, node, package, Wasm component, WIT, runtime, App Store, request/response, local/remote operations

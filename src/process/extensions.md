# Extensions

Extensions supplement and compliment Kinode processes.
Kinode processes have many features that make them good computational units, but they also have constraints.
Extensions remove the constraints (e.g., not all libraries can be built to Wasm) while maintaining the advantages (e.g., the integration with the Kinode Request/Response system).
The cost of extensions is that they are not as nicely bundled within the Kinode system: they must be run separately.

## What is an Extension?

Extensions are WebSocket clients that connect to a paired Kinode process to extend library, language, or hardware support.

Kinode processes are Wasm components, which leads to advantages and disadvantages.
The rest of the book (and in particular the [processes chapter](./processes.md)) discusses the advantages (e.g., integration with the Kinode Request/Response system and the capabilities security model).
Two of the main disadvantages are:
1. Only certain libraries and languages can be used.
2. Hardware accelerators like GPUs are not easily accessible.

Extensions solve both of these issues, since an extension runs natively.
Any language with any library supported by the bare metal host can be run as long as it can speak WebSockets.

## Downsides of Extensions

Extensions enable use cases that pure processes lack.
However, they come with a cost.
Processes are contained and managed by your Kinode, but extensions are not.
Extensions are independent servers that run alongside your Kinode.
They do not yet have a Kinode-native distribution channel.

As such, extensions should only be used when absolutely necessary.
Processes are more stable, maintainable, and easily upgraded.
Only write an extension if there is no other choice.

## How to Write an Extension?

Learn how to write an extension in the cookbook, [here](../cookbook/extensions.md).

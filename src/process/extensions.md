# Extensions

Extensions supplement and compliment Kinode processes.
Kinode processes have many features that make them good computational units, but they also have constraints.
Extensions remove the constraints while maintaining the advantages.
The cost of extensions is that they are not as nicely bundled within the Kinode system.

## What is an Extension?

Extensions are WebSocket clients that connect to a paired Kinode process to extend library, language, or hardware support.

The downsides, as well as the upsides, of Kinode processes stem from the fact that they are Wasm components.
The rest of the book (and in particular the [processes chapter](./processes.md)) discusses the advantages.
Two of the main disadvantages are:
1. Only certain libraries and languages can be used,
2. Hardware ccelerators like GPUs are not easily accessible.

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

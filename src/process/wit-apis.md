# WIT APIs

Kinode OS runs processes that are WebAssembly components, as discussed [elsewhere](./processes.md#wasm-and-kinode).
Two key advantages of WebAssembly components are

1. The declaration of types and functions using the cross-language Wasm Interface Type (WIT) language
2. The composibility of components.
See discussion [here](https://component-model.bytecodealliance.org/design/why-component-model.html).

Kinode processes make use of these two advantages.
A package — a group of processes, also referred to as an app — may define an API in WIT format.
The API is published alongside the package.
Other packages may then import and depend upon that API, and thus communicate with that package.
The publication of the API also allows for easy inspection by developers or by machines, e.g., LLM agents.

More than types can be published.
Because components are composable, packages may publish, along with the types in their API, library functions that may be of use in interacting with that package.
When set as as a dependency, these functions will be composed into new packages.
Libraries unassociated with packages can also be published and composed.

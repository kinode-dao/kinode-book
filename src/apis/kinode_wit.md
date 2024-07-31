# `kinode.wit`

Throughout this book, readers will see references to [WIT](https://component-model.bytecodealliance.org/design/wit.html), the [WebAssembly Component Model](https://github.com/WebAssembly/component-model).
WIT, or Wasm Interface Type, is a language for describing the types and functions that are available to a WebAssembly module.
In conjunction with the Component Model itself, WIT allows for the creation of WebAssembly modules that can be used as components in a larger system.
This standard has been under development for many years, and while still under construction, it's the perfect tool for building an operating-system-like environment for Wasm apps.

Kinode OS uses WIT to present a standard interface for Kinode processes.
This interface is a set of types and functions that are available to all processes.
It also contains functions (well, just a single function: `init()`) that processes must implement in order to compile and run in Kinode OS.
If one can generate WIT bindings in a language that compiles to Wasm, that language can be used to write Kinode processes.
So far, we've written Kinode processes in Rust, Javascript, Go, and Python.

To see exactly how to use WIT to write Kinode processes, see the [My First App](../my_first_app/chapter_1.md) chapter or the [Chess Tutorial](../chess_app/chess_engine.md).

To see `kinode.wit` for itself, see the [file in the GitHub repo](https://github.com/kinode-dao/kinode-wit/blob/master/kinode.wit).
Since this interface applies to all processes, it's one of the places in the OS where breaking changes are most likely to make an impact.
To that end, the version of the WIT file that a process uses must be compatible with the version of Kinode OS on which it runs.
Kinode intends to achieve perfect backwards compatibility upon first major release (1.0.0) of the OS and the WIT file.
After that point, since processes signal the version of the WIT file they use, subsequent updates can be made without breaking existing processes or needing to change the version they use.

## Types

[These 15 types](https://github.com/kinode-dao/kinode-wit/blob/758fac1fb144f89c2a486778c62cbea2fb5840ac/kinode.wit#L8-L106) make up the entirety of the shared type system between processes and the kernel.
Most types presented here are implemented in the [process standard library](../process_stdlib/overview.md) for ease of use.

## Functions

[These 16 functions](https://github.com/kinode-dao/kinode-wit/blob/758fac1fb144f89c2a486778c62cbea2fb5840ac/kinode.wit#L108-L190) are available to processes.
They are implemented in the kernel.
Again, the process standard library makes it such that these functions often don't need to be directly called in processes, but they are always available.
The functions are generally separated into 4 categories: system utilities, process management, capabilities management, and message I/O.
Future versions of the WIT file will certainly add more functions, but the categories themselves are highly unlikely to change.

System utilities are functions like `print_to_terminal`, whose role is to provide a way for processes to interact with the runtime in an idiosyncratic way.

Process management functions are used to adjust a processes' state in the kernel.
This includes its state-store and its on-exit behavior.
This category is also responsible for functions that give processes the ability to spawn and manage child processes.

Capabilities management functions relate to the capabilities-based security system imposed by the kernel on processes.
Processes must acquire and manage capabilities in order to perform tasks external to themselves, such as messaging another process or writing to a file.
See the [capabilities overview](../system/process/capabilities.md) for more details.

Lastly, message I/O functions are used to send and receive messages between processes.
Message-passing is the primary means by which processes communicate not only with themselves, but also with runtime modules which expose all kinds of I/O abilities.
For example, handling an HTTP request involves sending and receiving messages to and from the `http_server:disto:sys` runtime module.
Interacting with this module and others occurs through message I/O.

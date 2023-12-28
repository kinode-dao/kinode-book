# Capability-Based Security

TODO: after current phase of development, revisit and expand this section.

Capabilities are a security paradigm in which an ability that is usually handled as a *permission* (i.e. certain processes are allowed to perform an action if they are saved on an "access control list") are instead handled as a *token* (i.e. the process that possessses token can perform a certain action). These unforgable tokens (as enforced by the kernel) can be passed to other owners, held by a given process, and checked for.

In Uqbar, each process has an associated set of capabilities, which are represented internally as an arbitrary JSON object with a source process. The kernel abstracts away the procedure of checking if a capability is legitimate, using the running node's networking keys as a signing mechanism to ensure that capabilities are not forged.

Runtime processes, including the kernel itself, the filesystem, and the HTTP client, use capabilities to ensure that only the processes that should be able to access them can do so. For example, the filesystem has a capability that allows processes to read from a given file, and another that allows writing to a given file. For specific details, see the API reference for a given runtime process.

"System-level" capabilities like the above can only be given when a process is installed. A package uses the `manifest.json` file in its root directory to declare what capabilities its processes need. Upon install, the package manager (also referred to as "app store") surfaces these requested capabilities to the user, who can then choose to grant them or not.

"Userspace" capabilities, those *created by other processes*, can also be requested in a package manifest, though it's not guaranteed that the user will have installed the process that can grant the capability. Therefore, when a userspace process uses the capabilities system, it should have a way to grant capabilities through its IPC protocol. (TODO: link to example(s).)

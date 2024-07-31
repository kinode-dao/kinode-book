# Startup, Spindown, and Crashes

Along with learning how processes communicate, understanding the lifecycle paradigm of Kinode processes is essential to developing useful p2p applications.
Recall that a 'package' is a userspace construction that contains one or more processes.
The Kinode kernel is only aware of processes.
When a process is first initialized, its compiled Wasm code is loaded into memory and, if the code is valid, the process is added to the kernel's process table.
Then, the kernel starts the process by calling the `init` function (which is common to all processes).

This scenario is identical to when a process is re-initialized after being shut down. From the perspective of both the kernel and the process code, there is no difference.

## Defining Exit Behavior

In the capabilities chapter, we saw `manifest.json` used to request and grant capabilities to a process. Another field in the manifest is `on_exit`, which defines the behavior of the process when it exits.

There are three possible behaviors for a process when it exits:

- 1. `OnExit::None` - The process is not restarted and nothing happens.

- 2. `OnExit::Restart` - The process is restarted immediately.

- 3. `OnExit::Requests` - The process is not restarted, and a list of requests set by the process are fired off. These requests have the `source` and `capabilities` of the exiting process.

Once a process has been initialized it can exit in 4 ways:

- 1. Process code executes to completion -- `init()` returns.

- 2. Process code panics for any reason.

- 3. The kernel shuts it down via KillProcess call

- 4. The runtime shuts it down via graceful exit or crash.

In the event of a runtime exit the process often is best suited by restarting on the next boot. But this should be optional. This is the impetus for `OnExit::Restart` and `OnExit::None`. However, `OnExit::Requests` is also useful in this case, as the process can notify the appropriate services (which restarted, most likely) that it has exited.

If a process is killed by the kernel, it doesn't make sense to honor `OnExit::Restart`. This would reduce the strength of KillProcess and forces a full package uninstall to get it to stop running. Therefore, `OnExit::Restart` is treated as `OnExit::None` in this case only.

*NOTE: If a process crashes for a 'structural' reason, i.e. the process code leads directly to a panic, and uses `OnExit::Restart`, it will crash continuously until it is uninstalled or killed manually. Be careful of this!*

If a process executes to completion, its exit behavior is always honored.

Thus we can rewrite the three possible OnExit behaviors with their full accurate logic:

- 1. `OnExit::None` - The process is not restarted and nothing happens -- no matter what.

- 2. `OnExit::Restart` - The process is restarted immediately, unless it was killed by the kernel, in which case it is treated as `OnExit::None`.

- 3. `OnExit::Requests` - The process is not restarted, and a list of requests set by the process are fired off. These requests have the `source` and `capabilities` of the exiting process. If the target process of a given request in the list is no longer running, the request will be dropped.


### Implications

Here are some good practices for working with these behaviors:

1. When a process has `OnExit::Restart` as its behavior, it should be written in such a way that it can restart at any time. This means that the `init()` function should start by picking up where the process may have left off, for example, reading from a local database that the process uses, or re-establishing an ETH RPC subscription (and making sure to `get_logs` for any events that may have been missed!).

2. Processes that produce 'child' processes should handle the exit behavior of those children. A parent process should usually use `OnExit::Restart` as its behavior unless it intends to hand off the child processes to another process via some established API. A child process can use `None`, `Restart`, or `Requests`, depending on its needs.

- If a child process uses `OnExit::None`, the parent must be aware that the child could exit at any time and not notify the parent. This can be fine and easy to deal with if the parent has outstanding requests to the child and can assume failure on timeout, or if the work-product of the child is irrelevant to the continued operations of the parent.

- If a child process uses `OnExit::Restart`, the parent must be aware that the child will persist itself indefinitely. This is a natural fit for long-lived child processes which engage in cross-network activity and are themselves presenting a useful API. However, like `OnExit::None`, the parent will not be notified if the child process *is manually killed*. Again, the parent should be programmed to consider this.

- If a child process uses `OnExit::Requests`, it has the ability to notify the parent process when it exits. This is quite useful for child processes that create a work-product to return to the parent or if it is important that the parent do some action immediately upon the child's exit. Note that the programmer must *create* the requests in the child process. They can target any process, as long as the child process has the capability to message that target. The target will often simply be the parent process.

3. Requests made in `OnExit::Requests` must also comport to the capabilities requirements that applied to the process when it was alive.

4. If your processes does not have any places that it can panic, you don't have to worry about crash behavior, Rust is good for this :)

5. Parent processes "kill" child processes by building in a request type that the child will respond to by exiting, which the parent can then send. The kernel does not actually have any conception of hierarchical process relationships. The actual kernel `KillProcess` command requires root capabilities to use, and it is unlikely that your app will acquire those.

## Persisting State With Processes

Given that Kinodes can, comporting with the realities of the physical world, be turned off, a well-written process must withstand being shut down and re-initialized at any time.
This raises the question: how does a process persist information between initializations?
There are two ways: either the process can use the built-in `set_state` and `get_state` functions, or it can send data to a process that does this for them.

The first option is a maximally-simple way to write some bytes to disk (where they'll be backed up, if the node owner has configured that behavior).
The second option is vastly more general, because runtime modules, which can be messaged directly from custom userspace processes, offer any number of APIs.
So far, there are three modules built into Kinode OS that are designed for persisted data: a [filesystem](../files.md), a [key-value store, and a SQLite database](../databases.md).

Each of these modules offer APIs accessed via message-passing and write data to disk.
Between initializations of a process, this data remains saved, even backed up.
The process can then retrieve this data when it is re-initialized.
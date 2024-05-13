# Processes

Processes are independent pieces of Wasm code running on Kinode OS.
They can either be persistent, in which case they have in-memory state, or temporary, completing some specific task and returning.
They have access to long-term storage, like the filesystem or databases.
They can communicate locally and over the Kinode network.
They can access the internet via HTTP or WebSockets.
And these abilities can be controlled using a capabilities security model.

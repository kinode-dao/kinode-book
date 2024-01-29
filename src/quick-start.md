# Quick Start

Kinode OS is a decentralized operating system, peer-to-peer app framework, and node network designed to simplify the development and deployment of decentralized applications.

This Quick Start page is targeted at developers: if you want to get your hands dirty, continue [below](#run-two-fake-kinodes-and-chat-between-them).
A more detailed version of this Quick Start leads off the [My First Kinode Application](./my_first_app/chapter_1.md) section.

Otherwise:
* To learn about high-level concepts, start with the [Introduction](./intro.md), and work through the book in-order.
* To learn about how the system functions, start reading about [System Components](./processes.md).

## Run two fake Kinodes and chat between them

```
# Get Rust and `kit` Kinode development tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/kinode-dao/kit

# Start two fake nodes, each in a new terminal on ports 8080 and 8081:
## First new terminal:
kit boot-fake-node

## Second new terminal:
kit boot-fake-node --home /tmp/kinode-fake-node-2 --port 8081 --fake-node-name fake2.os

# Back in the original terminal that is not running a fake node:
## Create and build a chat app from a template:
kit new my_chat_app
kit build my_chat_app

## Load the chat app into each node & start it:
kit start-package my_chat_app
kit start-package my_chat_app --port 8081

## Chat between the nodes:
kit inject-message my_chat_app:my_chat_app:template.os '{"Send": {"target": "fake2.os", "message": "hello from the outside world"}}'
kit inject-message my_chat_app:my_chat_app:template.os '{"Send": {"target": "fake.os", "message": "replying from fake2.os using first method..."}}' --node fake2.os
kit inject-message my_chat_app:my_chat_app:template.os '{"Send": {"target": "fake.os", "message": "and second!"}}' -p 8081

# Or, from the terminal running one of the fake nodes:
## First fake node terminal:
m our@my_chat_app:my_chat_app:template.os '{"Send": {"target": "fake2.os", "message": "hello world"}}'

## Second fake node terminal:
m our@my_chat_app:my_chat_app:template.os '{"Send": {"target": "fake.os", "message": "wow, it works!"}}'
```

## Next steps

The first chapter of the [My First Kinode Application](./my_first_app/chapter_1.md) tutorial is a more detailed version of this Quick Start.
Alternatively, you can learn more about `kit` in the [`kit` documentation](./kit/kit.md).

If instead, you want to learn more about high-level concepts, start with the [Introduction](./intro.md) and work your way through the book in-order.

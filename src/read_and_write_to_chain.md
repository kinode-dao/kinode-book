# Read+Write to Chain

Kinode OS comes with a built-in provider module for Ethereum and other EVM chains/rollups.
This runtime module lives in [`eth:distro:sys`](https://github.com/kinode-dao/kinode/tree/main/kinode/src/eth) and is usable by any package that acquires the messaging capability for it.
In addition to allowing read/write connections directly to WebSocket RPC endpoints, the provider module can also connect via the Kinode networking protocol to other Kinodes and use their provider modules as a relay to an RPC endpoint (or to another Kinode, forming a relay chain).
The node must be configured to allow relay connections, which can be done with a public/private flag or explicit allow/deny list.

As with other runtime modules, processes should generally use the [kinode_process_lib](https://github.com/kinode-dao/process_lib) to interact with the RPC provider.
See [Reading Data from ETH](./cookbook/reading_data_from_eth.md) for an example of doing this in a process.
For more advanced or direct usage, such as configuring the provider module, see the [API Reference](./apis/eth_provider.md).

### Supported Chains

The provider module is capable of using any RPC endpoint that follows the [JSON-RPC API](https://ethereum.org/developers/docs/apis/json-rpc) that is used by Ethereum and most other EVM chains and rollups.
The runtime uses the [Alloy](https://github.com/alloy-rs) family of libraries to connect to WS RPC endpoints.
It does not currently support HTTP endpoints, as subscriptions are vastly preferable for many of the features that Kinode OS uses.

### Configuration

The [API Reference](./apis/eth_provider.md) demonstrates how to format requests to `eth:distro:sys` that adjust its config during runtime.
This includes adding and removing providers (whether other Kinodes or chain RPCs) and adjusting the permissions for other nodes to use this node as a relay.
However, most configuration is done in an optional file named `.eth-providers` inside the home folder of a node.
If this file is not present, a node will boot using the default providers hardcoded for testnet or mainnet, depending on where the node lives.
If it is present, the node will load in those providers and use them.
The file is a JSON object: a list of providers, with the following shape (example data):

```json
[
    {
        "chain_id": 1,
        "trusted": false,
        "provider": {
            "RpcUrl": "wss://ethereum.publicnode.com"
        }
    },
    {
        "chain_id": 11155111,
        "trusted": false,
        "provider": {
            "Node": {
                "use_as_provider": true,
                "kns_update": {
                    "name": "default-router-1.os",
                    "owner": "",
                    "node": "0xb35eb347deb896bc3fb6132a07fca1601f83462385ed11e835c24c33ba4ef73d",
                    "public_key": "0xb1b1cf23c89f651aac3e5fd4decb04aa177ab0ec8ce5f1d3877b90bb6f5779db",
                    "ip": "123.456.789.101",
                    "port": 9000,
                    "routers": []
                }
            }
        }
    }
]
```

One can see that the provider list includes both node-providers (other Kinodes that are permissioned for use as a relay) and url-providers (traditional RPC endpoints).
Nodes that wish to maximize their connectivity should supply themselves with url-providers, ideally trusted ones—they can even be running locally, with a light client for Ethereum such as [Helios](https://github.com/a16z/helios).
In fact, a future update to the provider module will likely integrate Helios, which will allow nodes to convert untrusted endpoints to trusted ones. This is the reason for the `trusted` flag in the provider object.

Lastly, note that the `kns_update` object must fully match the onchain PKI data for the given node, otherwise the two nodes will likely not be able to establish a connection.


# Sharing with the World

So, you've made a new process.
You've tested your code and are ready to share with friends, or perhaps just install across multiple nodes in order to do more testing.

First, it's a good idea to publish the code to a public repository.
This can be added to your package `metadata.json` like so:
```json
...
"website": "https://github.com/your_package_repo",
...
```
At a minimum you will need to publish the `metadata.json`.

Next, review all the data in [`pkg/manifest.json`](./chapter_1.md#pkgmanifestjson) and [`metadata.json`](./chapter_1.md#pkgmetadatajson).
The `package_name` field in `metadata.json` determines the name of the package.
The `publisher` field determines the name of the publisher (you!).

Once you're ready to share, it's quite easy.

If you are developing on a fake node, you'll have to boot a real one, then install this package locally in order to publish on the network, e.g.
```
kit s my_package
```


## Using the App Store GUI

Navigate to the App Store and follow the `Publish` flow, which will guide you through publishing your application.

## Using [`kit publish`](../kit/publish.md)

Alternatively, you can publish your application from the command-line using [`kit publish`](../kit/publish.md).
To do so, you'll either need to
1. Create a keystore.
2. Use a Ledger.
3. Use a Trezor.

The keystore is an encrypted wallet private key: the key that owns your publishing node.
[See below](#making-a-keystore) for discussion of how to create the keystore.
To use a hardware wallet, simply input the appropriate flag to `kit publish` (`-l` for Ledger or `-t` for Trezor).

In addition, you'll need an ETH RPC endpoint.
See the [`OPTIONAL: Acquiring an RPC API Key` section](../getting_started/login.md#starting-the-kinode) for a walkthrough of how to get an Alchemy API key.

### Making a Keystore

Keystores, also known as [Web3 Secret Storage](https://ethereum.org/en/developers/docs/data-structures-and-encoding/web3-secret-storage/), can be created in many ways; here, use [`foundry`](https://getfoundry.sh/)s `cast`.
First, [get `foundry`](https://getfoundry.sh/), and then run:
```
cast wallet import -i my_wallet
```
following the prompts to create your keystore named `my_wallet`.

### Running [`kit publish`](../kit/publish.md)

To publish your package, run:
```
kit publish --metadata-uri https://raw.githubusercontent.com/path/to/metadata.json --keystore-path ~/.foundry/keystores/my_wallet --rpc wss://opt-mainnet.g.alchemy.com/v2/<ALCHEMY_API_KEY> --real
```
and enter the password you created when making the keystore, here `my_wallet`.

Congratulations, your app is now live on the network!

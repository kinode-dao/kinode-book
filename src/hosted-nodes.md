# Hosted Nodes User Guide

Sybil Technologies runs a Kinode hosting service for users who do not want to run a node themselves.
These hosted Kinodes are useful for both end users and developers.
This guide is largely targeted at developers who want to develop Kinode applications using their hosted Kinode.
End users may also find the [Managing Your Kinode](#managing-your-kinode) section useful.

Here, `ssh` is used extensively.
This guide is specifically tailored to `ssh`s use for the Kinode hosting service.
A more expansive guide can be found [here](https://www.digitalocean.com/community/tutorials/ssh-essentials-working-with-ssh-servers-clients-and-keys).

## Managing Your Kinode

[Valet](https://valet.uncentered.systems) is the interface for managing your hosted node.
We plan to open-source the hosting code so there will be other hosting options in the future.
Once logged in, `Your Kinodes` will be displayed: clicking on the name of a Kinode will navigate to the homepage for that Kinode.
Clicking on the gear icon will open a modal with some information about the Kinode.
Click `Show advanced details` to reveal information for accessing your Kinode's terminal.

## Accessing Your Kinode's Terminal

As discussed in [Managing Your Kinode](#managing-your-kinode), navigate to:
1. [https://valet.uncentered.systems](https://valet.uncentered.systems)
2. `Your Kinodes`
3. Gear icon
4. `Show advanced details`

In the advanced details, note the `SSH Address` and `SSH Password`.

To access your Kinode remote instance, open a terminal and
```bash
ssh <SSH Address>
```
where `<SSH Address>` should be replaced with the one from your Valet advanced details.
You will be prompted for a password: copy-paste the `SSH Password`.

You should now have a different terminal prompt, indicating you have `ssh`d into the remote instance hosting your Kinode.

### Using SSH keys

Rather than typing in a password to create a SSH connection, you can use a keypair.

#### Generating Keypair

[How to generate a keypair](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent#generating-a-new-ssh-key)

#### `ssh-agent`

[How to use `ssh-agent` to store a keypair](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent#adding-your-ssh-key-to-the-ssh-agent)

#### SSH Config

[How to use `~/.ssh/config` to make SSH easier to use](https://www.digitalocean.com/community/tutorials/ssh-essentials-working-with-ssh-servers-clients-and-keys#defining-server-specific-connection-information)

#### Adding Public Key to Remote Node

[How to add the public key to a remote node to allow login with it](https://www.digitalocean.com/community/tutorials/ssh-essentials-working-with-ssh-servers-clients-and-keys#copying-your-public-ssh-key-to-a-server-with-ssh-copy-id)

## Using `kit` With Your Hosted Node

`kit` interacts with a Kinode through the Kinode's HTTP RPC.
However, Kinode limits HTTP RPC access to localhost — remote requests are rejected.
The local limit is a security measure, since the HTTP RPC allows injection of arbitrary messages with "root" capabilities.

To use `kit` with a hosted node, you need to create a SSH tunnel, which maps a port on your local machine to a port on the Kinode's remote host.
HTTP requests routed to that local port will then appear to the remote host as originating from its localhost.

It is recommended to use [`kit connect`](./kit/connect.md) to create and destroy a SSH tunnel.
Otherwise, you can also follow the instructions below to do it yourself.

Create a SSH tunnel like so (again, replacing [assumed values with those in your `advanced details`](#accessing-your-kinodes-terminal)):
```bash
ssh -L 9090:localhost:<HTTP port> <SSH address>
```
e.g.,
``` bash
ssh -L 9090:localhost:8099 kexampleuser@template.hosting.kinode.net
```

or, if you've added your host to your [`~/.ssh/config`](#ssh-config),
```bash
ssh -L 9090:localhost:<HTTP port> <Host>
```
You should see a `ssh` session open.
While this session is open, `kit` requests sent to `9090` will be routed to the remote Kinode, e.g.,
```
kit s foo -p 9090
```
will function the same as for a locally-hosted Kinode.

Closing the `ssh` connections with `Ctrl+D` or typing `exit` will close the tunnel.

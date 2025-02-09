# `kit connect`

`kit connect` is a thin wrapper over `ssh` to make creating SSH tunnels to remote nodes easy.

## Example Usage

Without any configuration, get your SSH Address from Valet, as discussed [here](../hosted-nodes.md#accessing-your-kinodes-terminal).
Then use
```
kit connect --host <SSH Address>
```
and paste in the node's SSH password when prompted.
You will be prompted for your password twice.
This is to first determine the port to create the SSH tunnel to, and then to create the tunnel.
You can also provide the port (Valet displays it as Local HTTP Port) and only be prompted for password once:
```
kit connect --host <SSH Address> --port <Valet Local HTTP Port>
```

It is recommended to [set up your SSH configuration on your local machine and the remote host](../hosted-nodes.md#using-ssh-keys).
Then `kit connect` usage looks like:
```
kit connect --host <Host>
```
where `<Host>` here is defined in your `~/.ssh/config` file.

To disconnect an SSH tunnel, use the `--disconnect` flag and the local port bound, by default, `9090`:
```
kit connect 9090 --disconnect
```

## Discussion

See discussion of why SSH tunnels are useful for development with `kit` [here](../hosted-nodes.md#using-kit-with-your-hosted-node).
Briefly, creating an SSH tunnel allows you to use `kit` with a remote hosted node in the same way you do with a local one.
Setting up your SSH configuration will make `kit connect` work better.
You can find instructions for doing so [here](../hosted-nodes.md#using-ssh-keys).

## Arguments

```
$ kit connect --help
Connect (or disconnect) an SSH tunnel to a remote server

Usage: kit connect [OPTIONS] [LOCAL_PORT]

Arguments:
  [LOCAL_PORT]  Local port to bind [default: 9090]

Options:
  -d, --disconnect        If set, disconnect an existing tunnel (default: connect a new tunnel)
  -o, --host <HOST>       Host URL/IP Kinode is running on (not required for disconnect)
  -p, --port <HOST_PORT>  Remote (host) port Kinode is running on
  -h, --help              Print help
```

# Frontend/UI Development

Kinode OS can serve a website or web app just like any HTTP webserver.
The preferred method is to upload your static assets on install by placing them in the `pkg` folder.
By convention, `kit` calls this folder `ui`, but you can call it anything.
You **must** place your `index.html` in the top-level folder.
The structure should look like this:

```
my_package
└── pkg
    └── ui (can have any name)
        ├── assets (can have any name)
        └── index.html
```

## Serving a Website

The simplest way to serve a UI is using the `serve_ui` function from `process_lib`:

```
serve_ui(&our, "ui").unwrap();
```

This will serve the `index.html` in the specified folder at the home path of your process.
If your process is called `my_package:my_package:template.os` and your Kinode is running locally on port 8080,
then the UI will be served at `http://localhost:8080/my_package:my_package:template.os`.

`serve_ui` takes two arguments: `&our` (&Address) and the directory where the UI assets are stored.

## Development without kit

The `kit` UI template uses the React framework compiled with Vite.
But you can use any UI framework as long as it generates an `index.html` and associated assets.
To make development easy, your setup should support a base URL and http proxying.

### Base URL

All processes in Kinode OS are namespaced by process name in the standard format of `process:package:publisher`.
So if your process is called `my_package:my_package:template.os`, then your process can only bind HTTP paths that start with `/my_package:my_package:template.os`.
Your UI should be developed and compiled with the base URL set to the appropriate process path.

#### Vite

In `vite.config.ts` (or `.js`) set `base` to your full process name, i.e. `base: '/my_package:my_package:template.os'`.

#### Create React App

In `package.json` set `homepage` to your full process name, i.e. `homepage: '/my_package:my_package:template.os'`.

### Proxying HTTP Requests

In UI development, it is very useful to proxy HTTP requests from the in-dev UI to your Kinode.
Below are some examples.

#### Vite

Follow the `server` entry in the (kit template)[https://github.com/kinode-dao/kit/blob/master/src/new/templates/ui/chat/ui/vite.config.ts#L31-L47] in your own `vite.config.ts`.

#### Create React App

In `package.json` set `proxy` to your Kinode's URL, i.e. `proxy: 'http://localhost:8080'`.

### Making HTTP Requests

When making HTTP requests in your UI, make sure to prepend your base URL to the request.
For example, if your base URL is `/my_package:my_package:template.os`, then a `fetch` request to `/my-endpoint` would look like this:

```
fetch('/my_package:my_package:template.os/my-endpoint')
```

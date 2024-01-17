# Frontends

Kinode OS can easily serve any webpage or web app developed with normal libraries and frameworks.

There are some specific endpoints, JS libraries, and process lib functions that are helpful for doing frontend development.

There are also some important considerations and "gotchas" that can happen when trying to do frontend development.

## /our & /our.js

Every node has both `/our` and `/our.js` endpoints.
`/our` returns the node's ID as a string like `'my-node'`.
`/our.js` returns a JS script that sets `window.our = { node: 'my-node' }`.
By convention, you can then easily set `window.our.process` either in your UI code or from a process-specific endpoint.
The frontend would then have `window.our` set for use in your code.

## JS Libraries

[@uqbar/client-encryptor-api](https://www.npmjs.com/package/@uqbar/client-encryptor-api) is a JavaScript library that simplifies establishing and handling a websocket connection to a node.

New libraries will be added as they become necessary or helpful.

## process_lib

In `process_lib::http` there is a function `serve_ui` which is helpful for serving your frontend from a node.
The two arguments to `serve_ui` are the current process' `&Address` and the name of the folder that contains your frontend.
The frontend folder must be placed in the `pkg` folder so that it is loaded into the virtual file system when the process is installed.
So if you put the frontend files in `pkg/ui` (with `index.html` at the top level!), the second argument to `serve_ui` would be `ui`.
Under the hood, `serve_ui` looks for `index.html` and binds it statically to your process' main route.
Then all of the other files in the frontend folder are bound to their respective paths.
For example, if your process is called `process:process:my-node`, then `index.html` would be bound to `/process:process:my-node`.
The file `pkg/ui/assets/index.js` would be bound to `/process:process:my-node/assets/index.js`

## Local Development and "gotchas"

When developing a frontend locally, particularly with a framework like React, it is helpful to proxy HTTP requests through to your node.
The `vite.config.ts` provided in the `kit` template, and listed in full below, has code to handle this proxying.

It is important to remember that the frontend will always have the process name as the first part of the HTTP path,
so all HTTP requests and file sources should start with the process name.
Many frontend JavaScript frameworks will handle this by default if you set the `base` or `baseUrl` properly.

In development, websocket connections can be more annoying to proxy, so it is often easier to simply hardcode the URL if in development.
See your framework documentation for how to check if you are in dev or prod.
The `kit` template already handles this for you.

Developing against a remote node is simple, you just have to change the proxy target in `vite.config.ts` to the URL of your node.
By default the template will target `http://localhost:8080`.

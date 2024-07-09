#!/usr/bin/env python3

import asyncio
import msgpack
import websockets

async def connect_websocket(
    uri="ws://localhost:8080/ws_server_with_reply:ws_server_with_reply:template.os",
    max_retries=5,
    delay_secs=0.5,
):
    attempt = 0
    while attempt < max_retries:
        try:
            return await websockets.connect(uri, ping_interval=None)
        except (
            websockets.ConnectionClosedError,
            websockets.InvalidURI,
            websockets.InvalidStatusCode,
        ) as e:
            attempt += 1
            await asyncio.sleep(delay_secs)

    raise Exception("Max retries exceeded, unable to connect.")

async def websocket_client():
    websocket = await connect_websocket()

    message = await websocket.recv()
    message = msgpack.unpackb(message, raw=False)
    message = message["WebSocketExtPushData"]
    m = msgpack.unpackb(bytes(message["blob"]), raw=False)
    print(f"Received from server: {m}")

    response = "Hello from client"
    response = msgpack.packb(response, use_bin_type=True)
    await websocket.send(response)

    websocket.close()

def main():
    asyncio.run(websocket_client())

if __name__ == "__main__":
    main()

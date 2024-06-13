#!/usr/bin/env python3

import asyncio
import msgpack
import websockets

async def websocket_client():
    uri = "ws://localhost:8080/ws_server_with_reply:ws_server_with_reply:template.os"

    # Connect to the WebSocket server
    async with websockets.connect(uri, ping_interval=None) as websocket:
        message = await websocket.recv()
        message = msgpack.unpackb(message, raw=False)
        message = message["WebSocketExtPushData"]
        m = msgpack.unpackb(bytes(message["blob"]), raw=False)
        print(f"Received from server: {m}")

        response = "Hello from client"
        response = msgpack.packb(response, use_bin_type=True)
        await websocket.send(response)

def main():
    asyncio.run(websocket_client())

if __name__ == "__main__":
    main()

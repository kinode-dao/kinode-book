#!/usr/bin/env python3

import asyncio
import websockets

async def connect_websocket(
    uri="ws://localhost:8080/ws-server:ws-server:template.os",
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
    print(f"Received from server: {message}")

    response = "Hello from client"
    await websocket.send(response)
    print(f"Sent to server: {response}")

    await websocket.close()

def main():
    asyncio.run(websocket_client())

if __name__ == "__main__":
    main()

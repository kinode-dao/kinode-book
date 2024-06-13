#!/usr/bin/env python3

import asyncio
import websockets

async def websocket_client():
    uri = "ws://localhost:8080/ws_server:ws_server:template.os"

    # Connect to the WebSocket server
    async with websockets.connect(uri, ping_interval=None) as websocket:
        # Wait for a message from the server
        message = await websocket.recv()
        print(f"Received from server: {message}")

        # Send a response message back to the server
        response = "Hello from client"
        await websocket.send(response)
        print(f"Sent to server: {response}")

def main():
    asyncio.run(websocket_client())

if __name__ == "__main__":
    main()

#!/usr/bin/env python3

import asyncio
import websockets

async def ws_handler(websocket, path, shutdown_event):
    try:
        await websocket.send("ack client connection")

        response = await websocket.recv()
        print(f"Received response from client: {response}")
    finally:
        await websocket.close()
        shutdown_event.set()

async def main():
    shutdown_event = asyncio.Event()

    async with websockets.serve(lambda ws, path: ws_handler(ws, path, shutdown_event), "localhost", 8765):
        print("Server started at ws://localhost:8765")

        await shutdown_event.wait()

        print("Shutting down server.")

if __name__ == '__main__':
    asyncio.run(main())

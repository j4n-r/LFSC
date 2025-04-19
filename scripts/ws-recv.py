#!usr/bin/env python
import json
import asyncio
import websockets

messages = [
    {
        "messageType": "IdMessage",
        "senderId": "d290f1ee-6c54-4b01-90e6-d701748f0851",
        "timestamp": "2025-04-19T13:00:00Z",
    },
]


async def send_messages(ws):
    try:
        for msg in messages:
            await ws.send(json.dumps(msg))
            await asyncio.sleep(1)
    except websockets.exceptions.ConnectionClosed:
        print("Send: Connection closed.")

async def receive_messages(ws):
    try:
        async for incoming in ws:
            print("Received:", incoming)
    except websockets.exceptions.ConnectionClosed:
        print("Receive: Connection closed.")

async def talk_and_listen():
    async with websockets.connect("ws://localhost:8080") as ws:
        send_task = asyncio.create_task(send_messages(ws))
        recv_task = asyncio.create_task(receive_messages(ws))
        await asyncio.gather(send_task, recv_task)

asyncio.run(talk_and_listen())

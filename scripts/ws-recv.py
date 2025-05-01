#!/usr/bin/env python
import asyncio
import json

import websockets

messages = [
    {
        "messageType": "IdMessage",
        "senderId": "203170c2-e811-44ba-a24f-a1e57d53b363",
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
    async with websockets.connect("ws://192.168.0.240:8080") as ws:
        send_task = asyncio.create_task(send_messages(ws))
        recv_task = asyncio.create_task(receive_messages(ws))
        await asyncio.gather(send_task, recv_task)


asyncio.run(talk_and_listen())

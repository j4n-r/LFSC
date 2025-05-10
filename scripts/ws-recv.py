#!/usr/bin/env python
import asyncio
import json
import websockets

messages = [
    {
        "messageType": "IdMessage",
        "senderId": "203170c2-e811-44ba-a24f-a1e57d53b363",  # test
        "timestamp": "2025-04-19T13:00:00Z",
    },
    # {
    #     "messageType": "ChatMessage",
    #     "payload": {
    #         "content": "I'm good, thanks! How about you?",
    #     },
    #     "meta": {
    #         "messageId": "6c9f3e7b-8c2d-4d6f-b654-abcdef654323",
    #         "conversationId": "c0000000-0000-0000-0000-00000000d001",  # DM between admin and test
    #         "senderId": "203170c2-e811-44ba-a24f-a1e57d53b363",
    #         "timestamp": "2025-04-19T13:00:10Z",
    #     },
    # },
    # {
    #     "messageType": "ChatMessage",
    #     "payload": {
    #         "content": "Hi everyone!",
    #     },
    #     "meta": {
    #         "messageId": "7d9f3e7b-8c2d-4d6f-b654-abcdef654324",
    #         "conversationId": "c0000000-0000-0000-0000-00000000g001",  # General group
    #         "senderId": "203170c2-e811-44ba-a24f-a1e57d53b363",
    #         "timestamp": "2025-04-19T13:01:05Z",
    #     },
    # },
]

async def send_messages(ws):
    for msg in messages:
        await ws.send(json.dumps(msg))
        await asyncio.sleep(1)

async def receive_messages(ws):
    async for incoming in ws:
        print("Received:", incoming)

async def talk_and_listen():
    async with websockets.connect("ws://192.168.0.240:8080") as ws:
        send_task = asyncio.create_task(send_messages(ws))
        recv_task = asyncio.create_task(receive_messages(ws))
        await asyncio.gather(send_task, recv_task)

asyncio.run(talk_and_listen())

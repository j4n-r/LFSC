#!/usr/bin/env python
import asyncio
import json
import websockets

messages = [
    {
        "messageType": "IdMessage",
        "senderId": "624f76c7-7b46-4309-8207-126317477e88",  # admin
        "timestamp": "2025-04-19T13:00:00Z",
    },
    {
        "messageType": "ChatMessage",
        "payload": {
            "content": "How are you today?",
        },
        "meta": {
            "conversationId": "c0000000-0000-0000-0000-00000000d001",  # DM between admin and test
            "senderId": "624f76c7-7b46-4309-8207-126317477e88",
            "timestamp": "2025-04-19T13:00:05Z",
        },
    },
    {
        "messageType": "ChatMessage",
        "payload": {
            "content": "Hello General group!",
        },
        "meta": {
            "conversationId": "c0000000-0000-0000-0000-00000000g001",  # General group
            "messageId": "5b9f3e7b-8c2d-4d6f-b654-abcdef654322",
            "senderId": "624f76c7-7b46-4309-8207-126317477e88",
            "timestamp": "2025-04-19T13:01:00Z",
        },
    },
]

async def send_messages(ws):
    for msg in messages:
        await ws.send(json.dumps(msg))
        print("send msg",json.dumps(msg))
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

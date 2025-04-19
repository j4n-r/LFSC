#!/usr/bin/env bash

jq . <<EOF | websocat ws://localhost:8080
{
  "message_type": "chat.message",
  "payload": {
    "targetType": "user",             
    "targetId":   "d290f1ee-6c54-4b01-90e6-d701748f0851",
    "content":    "Hello Bob!"
  },
  "meta": {
    "messageId": "3f8f2e9a-7d1c-4a5f-b321-abcdef123456",
    "senderId":  "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2025-04-19T13:00:00Z"
  }
}
EOF

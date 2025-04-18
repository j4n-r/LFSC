#!/usr/bin/env bash

jq . <<EOF | websocat ws://localhost:8080
{
  "sender_id":"550e8400-e29b-41d4-a716-446655440000",
  "receiver_id": "d290f1ee-6c54-4b01-90e6-d701748f0851",
  "text": "This is a message from alice to bob"
}
EOF

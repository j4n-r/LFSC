CREATE TABLE IF NOT EXISTS users (
    id          TEXT        PRIMARY KEY NOT NULL,
    username    TEXT                    NOT NULL,
    created_at  TEXT                    NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE TABLE IF NOT EXISTS devices (
    id          TEXT        PRIMARY KEY NOT NULL, -- UUID as TEXT
    user_id     TEXT        NOT NULL
        REFERENCES users(id) ON DELETE CASCADE, -- delete devices when user is deleted
    public_key  TEXT        NOT NULL,
    last_seen   TEXT        NOT NULL DEFAULT (CURRENT_TIMESTAMP) -- ISO8601 string
);

CREATE TABLE IF NOT EXISTS messages (
    id                TEXT    PRIMARY KEY   NOT NULL, -- UUID as TEXT
    sender_id         TEXT    NOT NULL
        REFERENCES users(id),
    target_type       TEXT    NOT NULL,  -- "user" or "group"
    target_id         TEXT    NOT NULL,  -- UUID as TEXT
    status            TEXT    NOT NULL,  -- "sent", "delivered", "buffered", "read"
    content           TEXT    NOT NULL,
    sent_from_client  TEXT    NOT NULL,  -- ISO8601 string
    sent_from_server  TEXT    NOT NULL   -- ISO8601 string
);

-- ───────────────────────────────────────────────────────────────────
-- Sample users
-- ───────────────────────────────────────────────────────────────────
INSERT INTO users (id, username, created_at) VALUES
  ('550e8400-e29b-41d4-a716-446655440000', 'alice', '2025-04-01 09:15:00'),
  ('d290f1ee-6c54-4b01-90e6-d701748f0851', 'bob',   '2025-04-02 11:30:00'),
  ('3fa85f64-5717-4562-b3fc-2c963f66afa6', 'carol', '2025-04-03 14:45:00');
  

-- ───────────────────────────────────────────────────────────────────
-- Sample devices
-- ───────────────────────────────────────────────────────────────────
INSERT INTO devices (id, user_id, public_key, last_seen) VALUES
  ('a3bb189e-8bf9-3888-9912-ace4e6543002', '550e8400-e29b-41d4-a716-446655440000',
      'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A...', '2025-04-10 08:20:00'),
  ('7c9e6679-7425-40de-944b-e07fc1f90ae7', '550e8400-e29b-41d4-a716-446655440000',
      'MIICZAIBADANBgkqhkiG9w0BAQEFAASC...', '2025-04-12 18:05:00'),
  ('123e4567-e89b-12d3-a456-426614174000', 'd290f1ee-6c54-4b01-90e6-d701748f0851',
      'MIGTAgEAMBMGByqGSM49AgEGCCqGSM49...', '2025-04-09 22:10:00'),
  ('f47ac10b-58cc-4372-a567-0e02b2c3d479', '3fa85f64-5717-4562-b3fc-2c963f66afa6',
      'MIIBCgKCAQEAn+X6Zt1TzJfPFlCqQ0...', '2025-04-11 12:00:00');


-- ───────────────────────────────────────────────────────────────────
-- Sample message_data
-- ───────────────────────────────────────────────────────────────────
INSERT INTO messages (
    id,
    sender_id,
    target_type,
    target_id,
    status,
    content,
    sent_from_client,
    sent_from_server
) VALUES
  (
    '11111111-1111-1111-1111-111111111111',
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'user',
    'd290f1ee-6c54-4b01-90e6-d701748f0851',  -- bob
    'sent',
    'This is a message from Alice to Bob',
    '2025-04-05 09:00:00',
    '2025-04-05 09:00:01'
  ),
  (
    '22222222-2222-2222-2222-222222222222',
    'd290f1ee-6c54-4b01-90e6-d701748f0851',  -- bob
    'user',
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'delivered',
    'Got your message, Alice!',
    '2025-04-05 09:01:30',
    '2025-04-05 09:01:32'
  ),
  (
    '33333333-3333-3333-3333-333333333333',
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'user',
    '3fa85f64-5717-4562-b3fc-2c963f66afa6',  -- carol
    'buffered',
    'Hey Carol, ping me when you’re back.',
    '2025-04-06 16:20:00',
    '2025-04-06 16:20:05'
  ),
  (
    '44444444-4444-4444-4444-444444444444',
    '3fa85f64-5717-4562-b3fc-2c963f66afa6',  -- carol
    'user',
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'read',
    'On it—see you at 5pm!',
    '2025-04-06 16:22:10',
    '2025-04-06 16:22:12'
  ),
  (
    '66666666-6666-6666-6666-666666666666',
    'd290f1ee-6c54-4b01-90e6-d701748f0851',  -- bob
    'group',
    '00000000-0000-0000-0000-000000000100',  -- some group ID
    'sent',
    'Hey everyone, meeting starts in 10 minutes.',
    '2025-04-07 19:50:00',
    '2025-04-07 19:50:02'
  );

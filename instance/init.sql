CREATE TABLE users (
  id  PRIMARY KEY NOT NULL, -- UUID as TEXT
  email TEXT UNIQUE NOT NULL,
  username TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  emailVerified BOOLEAN ,
  name TEXT,
  image TEXT,
  updated_at NOT NULL DEFAULT (CURRENT_TIMESTAMP),
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
    status            TEXT    NOT NULL CHECK (type IN ('sent', 'delivered','buffered','read')),  
    content           TEXT    NOT NULL,
    sent_from_client  TEXT    NOT NULL,  -- ISO8601 string
    sent_from_server  TEXT    NOT NULL   -- ISO8601 string
);

-- Groups, one-to-one DMs and small group-DMs
CREATE TABLE conversations (
    id           TEXT PRIMARY KEY NOT NULL,                -- UUID
    -- 'group' | 'dm' | 'group_dm'
    type         TEXT NOT NULL CHECK (type IN ('group','dm','group_dm')),
    owner_id     TEXT                                       -- NULL for pure DMs
        REFERENCES users(id) ON DELETE SET NULL,
    name         TEXT,                                     -- optional for DMs
    description  TEXT,
    image        TEXT,
    created_at   TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    updated_at   TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE TABLE conversation_members (
    conversation_id TEXT NOT NULL
        REFERENCES conversations(id) ON DELETE CASCADE,
    user_id         TEXT NOT NULL
        REFERENCES users(id) ON DELETE CASCADE,
    role            TEXT NOT NULL DEFAULT 'member',        -- 'owner'/'admin'/…
    joined_at       TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (conversation_id, user_id)
);

-- ───────────────────────────────────────────────────────────────────
-- Sample users
-- ───────────────────────────────────────────────────────────────────
-- run sc-admin "flask init-db" to create the admin and test user with hashed password

    -- users = [
    --     {
    --         "id": "624f76c7-7b46-4309-8207-126317477e88",
    --         "email": "admin@admin.com",
    --         "username": "admin",
    --         "password": "admin",
    --         "name": "admin",
    --     },
    --     {
    --         "id": "203170c2-e811-44ba-a24f-a1e57d53b363",
    --         "email": "test@test.com",
    --         "username": "test",
    --         "password": "test",
    --         "name": "test",
    --     },

-- ─────────────────────────────────────────────────────────────
-- DEVICES
-- ─────────────────────────────────────────────────────────────
INSERT INTO devices (id, user_id, public_key, last_seen) VALUES
  ('b1111111-1111-1111-1111-111111111111', '624f76c7-7b46-4309-8207-126317477e88',
   'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A...', '2025-04-15 09:10:00'),
  ('b2222222-2222-2222-2222-222222222222', '203170c2-e811-44ba-a24f-a1e57d53b363',
   'MIICZAIBADANBgkqhkiG9w0BAQEFAASC...', '2025-04-15 09:12:00');

-- ─────────────────────────────────────────────────────────────
-- CONVERSATIONS  (DM between admin & test + example group)
-- ─────────────────────────────────────────────────────────────
-- 1) direct message (DM)
INSERT INTO conversations (id, type, created_at)
VALUES ('c0000000-0000-0000-0000-00000000d001', 'dm', '2025-04-15 09:15:00');

-- 2) demo group owned by admin
INSERT INTO conversations (id, type, owner_id, name, created_at)
VALUES ('c0000000-0000-0000-0000-00000000g001', 'group',
        '624f76c7-7b46-4309-8207-126317477e88', 'General', '2025-04-15 09:20:00');

-- conversation_members
INSERT INTO conversation_members (conversation_id, user_id, role, joined_at) VALUES
  -- DM
  ('c0000000-0000-0000-0000-00000000d001', '624f76c7-7b46-4309-8207-126317477e88', 'member', '2025-04-15 09:15:00'),
  ('c0000000-0000-0000-0000-00000000d001', '203170c2-e811-44ba-a24f-a1e57d53b363', 'member', '2025-04-15 09:15:00'),
  -- Group
  ('c0000000-0000-0000-0000-00000000g001', '624f76c7-7b46-4309-8207-126317477e88', 'owner',  '2025-04-15 09:20:00'),
  ('c0000000-0000-0000-0000-00000000g001', '203170c2-e811-44ba-a24f-a1e57d53b363', 'member', '2025-04-15 09:22:00');

-- ─────────────────────────────────────────────────────────────
-- MESSAGES (admin ⇄ test DM + group notice)
-- ─────────────────────────────────────────────────────────────
INSERT INTO messages (id, sender_id, target_type, target_id,
                      status, content, sent_from_client, sent_from_server)
VALUES
  -- DM: admin → test
  ('m1111111-1111-1111-1111-111111111111',
   '624f76c7-7b46-4309-8207-126317477e88',  -- admin
   'user', '203170c2-e811-44ba-a24f-a1e57d53b363',
   'sent', 'Hi test, welcome aboard!',
   '2025-04-15 09:16:00', '2025-04-15 09:16:01'),

  -- DM: test → admin
  ('m2222222-2222-2222-2222-222222222222',
   '203170c2-e811-44ba-a24f-a1e57d53b363',  -- test
   'user', '624f76c7-7b46-4309-8207-126317477e88',
   'delivered', 'Thanks! Glad to join.',
   '2025-04-15 09:17:10', '2025-04-15 09:17:11'),

  -- Group: admin → General
  ('m3333333-3333-3333-3333-333333333333',
   '624f76c7-7b46-4309-8207-126317477e88',  -- admin
   'group', 'c0000000-0000-0000-0000-00000000g001',
   'sent', 'Stand-up starts in 5 min.',
   '2025-04-15 09:25:00', '2025-04-15 09:25:02');

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,   
    username TEXT,
    created_at TIMESTAMP    
);

CREATE TABLE IF NOT EXISTS devices (
    id UUID PRIMARY KEY,
    user_id UUID,
    public_key TEXT,
    last_seen TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS message_data (
    id UUID PRIMARY KEY,
    sender_id UUID,
    receiver_id UUID,
    status TEXT,
    sent_at TIMESTAMP,
    FOREIGN KEY(sender_id)   REFERENCES users(id),
    FOREIGN KEY(receiver_id) REFERENCES users(id)
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
INSERT INTO message_data (id, sender_id, receiver_id, status, sent_at) VALUES
  ('11111111-1111-1111-1111-111111111111',
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'd290f1ee-6c54-4b01-90e6-d701748f0851',  -- bob
    'sent', '2025-04-05 09:00:00'),
  ('22222222-2222-2222-2222-222222222222',
    'd290f1ee-6c54-4b01-90e6-d701748f0851',  -- bob
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'delivered', '2025-04-05 09:01:30'),
  ('33333333-3333-3333-3333-333333333333',
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    '3fa85f64-5717-4562-b3fc-2c963f66afa6',  -- carol
    'sent', '2025-04-06 16:20:00'),
  ('44444444-4444-4444-4444-444444444444',
    '3fa85f64-5717-4562-b3fc-2c963f66afa6',  -- carol
    '550e8400-e29b-41d4-a716-446655440000',  -- alice
    'read', '2025-04-06 16:22:10'),
  ('55555555-5555-5555-5555-555555555555',
    'd290f1ee-6c54-4b01-90e6-d701748f0851',  -- bob
    '3fa85f64-5717-4562-b3fc-2c963f66afa6',  -- carol
    'sent', '2025-04-07 20:00:00');

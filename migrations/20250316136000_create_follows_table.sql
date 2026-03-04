CREATE TABLE IF NOT EXISTS follows (
    follower_id  UUID        NOT NULL REFERENCES users(unid) ON DELETE CASCADE,
    following_id UUID        NOT NULL REFERENCES users(unid) ON DELETE CASCADE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (follower_id, following_id),
    CHECK (follower_id <> following_id)
);

-- Seed: Bob and Charlie follow Alice; Alice follows Bob
INSERT INTO follows (follower_id, following_id) VALUES
    ('00000000-0000-0000-0001-000000000002', '00000000-0000-0000-0001-000000000001'),
    ('00000000-0000-0000-0001-000000000003', '00000000-0000-0000-0001-000000000001'),
    ('00000000-0000-0000-0001-000000000001', '00000000-0000-0000-0001-000000000002');

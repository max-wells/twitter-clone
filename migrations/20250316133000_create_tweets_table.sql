CREATE TABLE IF NOT EXISTS tweets (
    unid       UUID          PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    author_id  UUID          NOT NULL REFERENCES users(unid) ON DELETE CASCADE,
    content    VARCHAR(280)  NOT NULL,
    created_at TIMESTAMPTZ   NOT NULL DEFAULT now()
);

INSERT INTO tweets (unid, author_id, content, created_at) VALUES
    ('00000000-0000-0000-0002-000000000001',
     '00000000-0000-0000-0001-000000000001',
     'Just shipped my first Leptos app 🦀 Rust + WASM is unreal.',
     now() - interval '5 minutes'),
    ('00000000-0000-0000-0002-000000000002',
     '00000000-0000-0000-0001-000000000002',
     'The borrow checker is not your enemy — it''s your strictest friend.',
     now() - interval '1 hour'),
    ('00000000-0000-0000-0002-000000000003',
     '00000000-0000-0000-0001-000000000003',
     'Tauri + Leptos is the stack of the future. Desktop apps in pure Rust 🚀',
     now() - interval '3 hours');

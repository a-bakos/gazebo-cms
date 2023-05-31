CREATE TABLE IF NOT EXISTS gb_post_meta (
    post_meta_id serial PRIMARY KEY,
    post_id INTEGER DEFAULT NULL,
    meta_key VARCHAR(255) NOT NULL,
    meta_value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS gb_account_meta (
    account_meta_id serial PRIMARY KEY,
    user_id INTEGER DEFAULT NULL,
    meta_key VARCHAR(255) NOT NULL,
    meta_value TEXT
);

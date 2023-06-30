CREATE TABLE IF NOT EXISTS gb_log (
    id serial PRIMARY KEY,
    user_id INTEGER NOT NULL,
    user_email VARCHAR(100) NOT NULL,
    event_code INTEGER NOT NULL,
    event_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    title VARCHAR(255) NOT NULL,
    description TEXT DEFAULT NULL
);
CREATE TABLE IF NOT EXISTS gb_log (
    id serial PRIMARY KEY,
    account_id INTEGER NOT NULL,
    account_email VARCHAR(100) DEFAULT NULL,
    event_code INTEGER NOT NULL,
    event_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    subject_id INTEGER NOT NULL,
    subject_url VARCHAR(255) DEFAULT NULL,
    subject_title VARCHAR(255) DEFAULT NULL,
    subject_description TEXT DEFAULT NULL,
    subject_from TEXT DEFAULT NULL,
    subject_to TEXT DEFAULT NULL
);
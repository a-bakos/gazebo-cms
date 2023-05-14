CREATE TABLE IF NOT EXISTS gb_posts (
    id serial PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
)

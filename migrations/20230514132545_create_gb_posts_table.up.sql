CREATE TABLE IF NOT EXISTS gb_posts (
    id serial PRIMARY KEY,
    author_id INTEGER NOT NULL,
    parent_id INTEGER DEFAULT NULL,
    date_publish TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    date_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    slug VARCHAR(255) NOT NULL,
    --post_type VARCHAR(20) DEFAULT 'post', -- type is not needed if table is called "posts"
    title VARCHAR(255) NOT NULL,
    excerpt TEXT DEFAULT NULL,
    content TEXT NOT NULL,
    password VARCHAR(255) DEFAULT NULL,
    status VARCHAR(20) DEFAULT 'draft'
    --tags INTEGER[] DEFAULT NULL -- todo
);
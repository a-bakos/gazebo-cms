CREATE TABLE IF NOT EXISTS gb_media (
    id serial PRIMARY KEY,
    uploader INTEGER NOT NULL,
    attached_to INTEGER[] DEFAULT NULL,
    date_publish TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    date_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    slug VARCHAR(255) NOT NULL,
    --post_type VARCHAR(20) DEFAULT 'media', -- type is not needed if table is called "media"
    title VARCHAR(255) NOT NULL,
    description TEXT DEFAULT NULL,
    --status VARCHAR(20) DEFAULT 'attached' -- status is not needed if we have attached_to
    --tags INTEGER[] DEFAULT NULL
);
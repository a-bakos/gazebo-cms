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
    -- Mime type ??
    -- File/Image URL: The URL or file path of the image is essential for referencing and displaying the image on web pages. It can be stored as a separate field or derived from the uploaded file.
    -- Dimensions: Including the width and height of the image can be useful for layout and responsive design purposes. These dimensions can be automatically extracted from the image file metadata or calculated when the image is uploaded.
);
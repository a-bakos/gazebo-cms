INSERT INTO gb_posts (author_id, title, content, post_type, slug, status)
SELECT 1, 'Hello, Gazebo!', 'First demo post for your Gazebo CMS.', 'post', 'hello-gazebo', 'publish'
UNION ALL
SELECT 1, 'A CMS experiment project', 'Second demo post for your Gazebo CMS.', 'post', 'cms-experiment', 'draft'
UNION ALL
SELECT 1, 'Inspired by WordPress', 'Third demo post for your Gazebo CMS.', 'post', 'inspired-by-wp', 'private'
WHERE NOT EXISTS (SELECT id FROM gb_posts WHERE title IN ('Hello, Gazebo!', 'A CMS experiment project', 'Inspired by WordPress'))
RETURNING id;

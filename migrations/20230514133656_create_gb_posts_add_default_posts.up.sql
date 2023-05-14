INSERT INTO gb_posts (title, content)
SELECT 'Hello, Gazebo!', 'First demo post for your Gazebo CMS.'
UNION ALL
SELECT 'A CMS experiment project', 'Second demo post for your Gazebo CMS.'
UNION ALL
SELECT 'Inspired by WordPress', 'Third demo post for your Gazebo CMS.'
WHERE NOT EXISTS (SELECT id FROM gb_posts WHERE title IN ('Hello, Gazebo!', 'A CMS experiment project', 'Inspired by WordPress'))
RETURNING id;

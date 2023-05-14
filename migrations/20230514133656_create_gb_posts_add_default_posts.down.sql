-- The *_down.sql commands are executed when you run a database migration rollback to undo the changes made by a previous migration.
-- For example, if you have applied a migration that creates a new table in your database, and you want to roll back that migration,
-- you would execute the sqlx migrate rollback command.
-- This would run the *_down.sql script associated with the migration and remove the table from the database.
-- The *_down.sql script should reverse the changes made by the corresponding *_up.sql script in the migration,
-- to ensure that the rollback operation works correctly.

DELETE FROM gb_posts WHERE title IN (
    'Hello, Gazebo!',
    'A CMS experiment project',
    'Inspired by WordPress'
);

DROP TABLE IF EXISTS Users;
DROP FUNCTION IF EXISTS update_change_timestamp_column CASCADE;
DROP TRIGGER IF EXISTS update_users_change_timestamp ON Users CASCADE;

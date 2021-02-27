PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE messages(id integer primary key, from_chat_id integer not null, message_id integer not null);
CREATE TABLE info(last_message integer not null);
INSERT INTO info VALUES(1614395326);
COMMIT;

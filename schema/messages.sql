PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE messages(id integer primary key, from_chat_id integer not null, message_id integer not null);
INSERT INTO messages VALUES(1,878897433,32235);
INSERT INTO messages VALUES(2,878897433,32240);
INSERT INTO messages VALUES(3,878897433,32251);
CREATE TABLE info(last_message integer not null);
INSERT INTO info VALUES(1614395326);
COMMIT;

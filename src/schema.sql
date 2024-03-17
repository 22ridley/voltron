--INITIALIZING TABLE STRUCTURE:

-- One table containing users, and one table specifying group information
CREATE TABLE users (user_name varchar(255), privilege int, class_id int, group_id int);

-- Adding root instructor and one sampe instructor
-- Also adding sample students
INSERT INTO users (user_name, privilege, class_id, group_id) VALUES ("admin", 2, -1, -1)
INSERT INTO users (user_name, privilege, class_id, group_id) VALUES ("professor", 1, 0, -1)
INSERT INTO users (user_name, privilege, class_id, group_id) VALUES ("sarah", 0, 0, 0)
INSERT INTO users (user_name, privilege, class_id, group_id) VALUES ("naomi", 0, 0, 0)
INSERT INTO users (user_name, privilege, class_id, group_id) VALUES ("elena", 0, 0, 1)
INSERT INTO users (user_name, privilege, class_id, group_id) VALUES ("zoe", 0, 0, 1)
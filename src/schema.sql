--INITIALIZING TABLE STRUCTURE:

-- One table containing users, and one table specifying group information
CREATE TABLE users (user_name varchar(255), privilege int, group_id int);
CREATE TABLE student_groups (group_id int, code text, PRIMARY KEY (group_id));

-- Adding root instructor and one sampe instructor
-- Also adding sample students
INSERT INTO users (user_name, privilege, group_id) VALUES ("admin", 2, -1)
INSERT INTO users (user_name, privilege, group_id) VALUES ("professor", 1, -1)
INSERT INTO users (user_name, privilege, group_id) VALUES ("sarah", 0, 0)
INSERT INTO users (user_name, privilege, group_id) VALUES ("naomi", 0, 0)
INSERT INTO users (user_name, privilege, group_id) VALUES ("elena", 0, 1)
INSERT INTO users (user_name, privilege, group_id) VALUES ("zoe", 0, 1)

-- Adding sample group information
INSERT INTO student_groups (group_id, code) VALUES (0, "//Write your code here!")
INSERT INTO student_groups (group_id, code) VALUES (1, "//Write your code here!")
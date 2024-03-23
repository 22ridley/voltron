--INITIALIZING TABLE STRUCTURE:

-- One table containing users, and one table specifying group information
CREATE TABLE users (user_name varchar(255), email varchar(255), privilege int, class_id int, group_id int);

-- Adding root instructor and one sampe instructor
-- Also adding sample students
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("admin", "22ridleysk@gmail.com", 2, -1, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("professor", "sarah_ridley@brown.edu", 1, 0, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("sarah kate", "sarah.kate.ridley@gmail.com", 0, 0, 0)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("anon", "n7pyqu7lznggibcj@gmail.com", 0, 0, 0)
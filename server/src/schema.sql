--INITIALIZING TABLE STRUCTURE:

-- One table containing users, and one table specifying group information
CREATE TABLE users (user_name varchar(255), email varchar(255), privilege int, class_id int, group_id int);

-- Adding root instructor and one sampe instructor
-- Also adding sample students
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Admin", "22ridleysk@gmail.com", 2, -1, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Prof. Sarah", "sarah_ridley@brown.edu", 1, 0, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Prof. Voltron", "prof.voltron@gmail.com", 1, 1, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Sarah Kate", "sarah.kate.ridley@gmail.com", 0, 0, 0)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Anon", "n7pyqu7lznggibcj@gmail.com", 0, 0, 0)
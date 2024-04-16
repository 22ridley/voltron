--INITIALIZING TABLE STRUCTURE:

-- One table containing users, and one table specifying group information
-- privilege 0 => student, 1 => instructor, 2 => admin
CREATE TABLE users (user_name varchar(255), email varchar(255), privilege int, class_id int, group_id int, PRIMARY KEY (email));

-- Adding root instructor and one sample instructor
-- Also adding sample students
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Admin", "22ridleysk@gmail.com", 2, -1, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Prof. S", "sarah_ridley@brown.edu", 1, 0, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Prof. V", "prof.voltron@gmail.com", 1, 1, -1)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Sarah Kate", "sarah.kate.ridley@gmail.com", 0, 0, 0)
INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES ("Anon", "n7pyqu7lznggibcj@gmail.com", 0, 0, 0)

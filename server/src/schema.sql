--INITIALIZING TABLE STRUCTURE:

-- Table containing users
CREATE TABLE user (user_id int NOT NULL AUTO_INCREMENT, user_name varchar(255), email varchar(255), privilege int, PRIMARY KEY (user_id));
-- Table containing class information
CREATE TABLE class (class_id int NOT NULL AUTO_INCREMENT, class_name varchar(255), instructor_id int, PRIMARY KEY(class_id), FOREIGN KEY(instructor_id) REFERENCES user(user_id))
-- Table containing group information
CREATE TABLE group (group_id int NOT NULL AUTO_INCREMENT, group_name varchar(255), class_id int, PRIMARY KEY(group_id), FOREIGN KEY(class_id) REFERENCES class(class_id))
-- Table containing student enrollment information
CREATE TABLE enroll (student_id int, class_id int, group_id int, PRIMARY KEY(student_id), FOREIGN KEY(student_id) REFERENCES user(user_id), FOREIGN KEY(group_id) REFERENCES class(class_id))

-- Adding root instructor and sample instructors
-- Also adding sample students
INSERT INTO user (user_name, email, privilege) VALUES ("Admin", "22ridleysk@gmail.com", 2)        -- 1
INSERT INTO user (user_name, email, privilege) VALUES ("Prof. S", "sarah_ridley@brown.edu", 1)    -- 2
INSERT INTO user (user_name, email, privilege) VALUES ("Prof. V", "voltron@gmail.com", 1)         -- 3
INSERT INTO user (user_name, email, privilege) VALUES ("Sarah", "sarah.kate.ridley@gmail.com", 0) -- 4
INSERT INTO user (user_name, email, privilege) VALUES ("Anon", "n7pyqu7lznggibcj@gmail.com", 0)   -- 5

INSERT INTO class (class_name, instructor_id) VALUES ("CSCI 0300", 2) -- 1
INSERT INTO class (class_name, instructor_id) VALUES ("CSCI 2390", 3) -- 2

INSERT INTO group (group_name, class_id) VALUES ("Group 1", 1)    -- 1
INSERT INTO group (group_name, class_id) VALUES ("Group 2 :)", 1) -- 2

INSERT INTO enroll (student_id, class_id, group_id) VALUES (4, 1, 1)
INSERT INTO enroll (student_id, class_id, group_id) VALUES (5, 1, 2)
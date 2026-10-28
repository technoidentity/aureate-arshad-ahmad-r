
-- Practice 2: Student marks and aggregate functions

CREATE TABLE IF NOT EXISTS students (
    id INTEGER PRIMARY KEY,
    name TEXT,
    marks INTEGER
);

INSERT INTO students (name, marks) VALUES ('Arun', 80);
INSERT INTO students (name, marks) VALUES ('Priya', 90);
INSERT INTO students (name, marks) VALUES ('Rahul', 75);
INSERT INTO students (name, marks) VALUES ('Sneha', 85);
INSERT INTO students (name, marks) VALUES ('Kiran', 70);

-- Display all students
SELECT * FROM students;

-- Calculate total marks
SELECT SUM(marks) AS total_marks FROM students;

-- Calculate average marks
SELECT AVG(marks) AS average_marks FROM students;

-- Find highest marks
SELECT MAX(marks) AS highest_marks FROM students;

-- Find lowest marks
SELECT MIN(marks) AS lowest_marks FROM students;

-- Count students
SELECT COUNT(*) AS total_students FROM students;

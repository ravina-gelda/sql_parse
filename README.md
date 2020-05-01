# sql_parse
## Examples of DML commands  in SQL Server are
* SELECT – This SQL DML command select records or data from a table

* INSERT – Insert data into a database table.
* UPDATE – This SQL DML command will update existing records within a table
* DELETE – Delete unwanted records from a table 

### example queries to be supported
* SELECT * FROM Students WHERE StudentId = 6;
* UPDATE Students
SET DepartmentId = 3 
WHERE StudentId = 6;
* DELETE FROM Students WHERE StudentId = 11 OR StudentId = 12;
* INSERT INTO Students(StudentId, StudentName, DepartmentId, DateOfBirth)
              VALUES(11, 'Ahmad', 4, '1997-10-12');
*INSERT INTO Students VALUES(12, 'Aly', 4, '1996-10-12');
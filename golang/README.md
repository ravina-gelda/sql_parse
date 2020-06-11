
# Requirements

## Golang installation steps: https://golang.org/doc/install
 * Dowload Go from : https://golang.org/dl/
 * Choose the archive file appropriate for your installation. For instance, if you are installing Go version 1.2.1 for 64-bit x86 on Linux, the archive you want is called go1.2.1.linux-amd64.tar.gz.
 I have installed: ``` install go version go1.14.2 windows/amd64 ```
 * Add /usr/local/go/bin to the PATH environment variable. You can do this by adding this line to your /etc/profile (for a system-wide installation) or $HOME/.profile: 
 ```export PATH=$PATH:/usr/local/go/bin ```
 * Note: changes made to a profile file may not apply until the next time you log into your computer. To apply the changes immediately, just run the shell commands directly or execute them from the profile using a command such as source $HOME/.profile.
## Test your Go installation
* Create a file hello.go
```
package main

import "fmt"

func main() {
	fmt.Printf("hello, world\n")
}
```
* Then run it with the Go tool
```
C:\Users\Gopher\go\src\hello> go run hello.go
```
** output:
```hello, world```

## Execution details of the sqlparse/golang
* Download the repository
* Setup the path for the folder golang and run SQL_parser_main.go with the command below:
``` go run SQL_parser_main.go ```
* Test the test cases with the command: 
** set the path of command line to SQL_parser_golang/parser and run the below command:
``` go test -v ```
## Golang SQL Parser Implementation
## Examples of DML commands  in SQL Server are
* SELECT – This SQL DML command select records or data from a table

* INSERT – Insert data into a database table.
* UPDATE – This SQL DML command will update existing records within a table
* DELETE – Delete unwanted records from a table 

### example queries to be supported
### Example: SELECT works

```
query, err := sqlparser.Parse(`SELECT a FROM 'b'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: []
	Updates: map[]
	Inserts: []
	Fields: [a]
}
```

### Example: SELECT works with lowercase

```
query, err := sqlparser.Parse(`select a fRoM 'b'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: []
	Updates: map[]
	Inserts: []
	Fields: [a]
}
```

### Example: SELECT many fields works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: []
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT with WHERE with = works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a = ''`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: ,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT with WHERE with < works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a < '1'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Lt,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT with WHERE with <= works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a <= '1'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Lte,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT with WHERE with > works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a > '1'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Gt,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT with WHERE with >= works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a >= '1'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Gte,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT with WHERE with != works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a != '1'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Ne,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: SELECT * works

```
query, err := sqlparser.Parse(`SELECT * FROM 'b'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: []
	Updates: map[]
	Inserts: []
	Fields: [*]
}
```

### Example: SELECT a, * works

```
query, err := sqlparser.Parse(`SELECT a, * FROM 'b'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: []
	Updates: map[]
	Inserts: []
	Fields: [a *]
}
```

### Example: SELECT with WHERE with two conditions using AND works

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a != '1' AND b = '2'`)

query.Query {
	Type: Select
	TableName: b
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Ne,
            Operand2: 1,
            Operand2IsField: false,
        }
        {
            Operand1: b,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: 2,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: [a c d]
}
```

### Example: UPDATE works

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b = 'hello' WHERE a = '1'`)

query.Query {
	Type: Update
	TableName: a
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[b:hello]
	Inserts: []
	Fields: []
}
```

### Example: UPDATE with multiple SETs works

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b = 'hello', c = 'bye' WHERE a = '1'`)

query.Query {
	Type: Update
	TableName: a
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[b:hello c:bye]
	Inserts: []
	Fields: []
}
```

### Example: UPDATE with multiple SETs and multiple conditions works

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b = 'hello', c = 'bye' WHERE a = '1' AND b = '789'`)

query.Query {
	Type: Update
	TableName: a
	Conditions: [
        {
            Operand1: a,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: 1,
            Operand2IsField: false,
        }
        {
            Operand1: b,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: 789,
            Operand2IsField: false,
        }]
	Updates: map[b:hello c:bye]
	Inserts: []
	Fields: []
}
```

### Example: DELETE with WHERE works

```
query, err := sqlparser.Parse(`DELETE FROM 'a' WHERE b = '1'`)

query.Query {
	Type: Delete
	TableName: a
	Conditions: [
        {
            Operand1: b,
            Operand1IsField: true,
            Operator: Eq,
            Operand2: 1,
            Operand2IsField: false,
        }]
	Updates: map[]
	Inserts: []
	Fields: []
}
```

### Example: INSERT works

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b) VALUES ('1')`)

query.Query {
	Type: Insert
	TableName: a
	Conditions: []
	Updates: map[]
	Inserts: [[1]]
	Fields: [b]
}
```

### Example: INSERT with multiple fields works

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b,c,    d) VALUES ('1','2' ,  '3' )`)

query.Query {
	Type: Insert
	TableName: a
	Conditions: []
	Updates: map[]
	Inserts: [[1 2 3]]
	Fields: [b c d]
}
```

### Example: INSERT with multiple fields and multiple values works

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b,c,    d) VALUES ('1','2' ,  '3' ),('4','5' ,'6' )`)

query.Query {
	Type: Insert
	TableName: a
	Conditions: []
	Updates: map[]
	Inserts: [[1 2 3] [4 5 6]]
	Fields: [b c d]
}
```



### Example: empty query fails

```
query, err := sqlparser.Parse(``)

query type cannot be empty
```

### Example: SELECT without FROM fails

```
query, err := sqlparser.Parse(`SELECT`)

table name cannot be empty
```

### Example: SELECT without fields fails

```
query, err := sqlparser.Parse(`SELECT FROM 'a'`)

at SELECT: expected field to SELECT
```

### Example: SELECT with comma and empty field fails

```
query, err := sqlparser.Parse(`SELECT b, FROM 'a'`)

at SELECT: expected field to SELECT
```

### Example: SELECT with empty WHERE fails

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE`)

at WHERE: empty WHERE clause
```

### Example: SELECT with WHERE with only operand fails

```
query, err := sqlparser.Parse(`SELECT a, c, d FROM 'b' WHERE a`)

at WHERE: condition without operator
```

### Example: Empty UPDATE fails

```
query, err := sqlparser.Parse(`UPDATE`)

table name cannot be empty
```

### Example: Incomplete UPDATE with table name fails

```
query, err := sqlparser.Parse(`UPDATE 'a'`)

at WHERE: WHERE clause is mandatory for UPDATE & DELETE
```

### Example: Incomplete UPDATE with table name and SET fails

```
query, err := sqlparser.Parse(`UPDATE 'a' SET`)

at WHERE: WHERE clause is mandatory for UPDATE & DELETE
```

### Example: Incomplete UPDATE with table name, SET with a field but no value and WHERE fails

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b WHERE`)

at UPDATE: expected '='
```

### Example: Incomplete UPDATE with table name, SET with a field and = but no value and WHERE fails

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b = WHERE`)

at UPDATE: expected quoted value
```

### Example: Incomplete UPDATE due to no WHERE clause fails

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b = 'hello' WHERE`)

at WHERE: empty WHERE clause
```

### Example: Incomplete UPDATE due incomplete WHERE clause fails

```
query, err := sqlparser.Parse(`UPDATE 'a' SET b = 'hello' WHERE a`)

at WHERE: condition without operator
```

### Example: Empty DELETE fails

```
query, err := sqlparser.Parse(`DELETE FROM`)

table name cannot be empty
```

### Example: DELETE without WHERE fails

```
query, err := sqlparser.Parse(`DELETE FROM 'a'`)

at WHERE: WHERE clause is mandatory for UPDATE & DELETE
```

### Example: DELETE with empty WHERE fails

```
query, err := sqlparser.Parse(`DELETE FROM 'a' WHERE`)

at WHERE: empty WHERE clause
```

### Example: DELETE with WHERE with field but no operator fails

```
query, err := sqlparser.Parse(`DELETE FROM 'a' WHERE b`)

at WHERE: condition without operator
```

### Example: Empty INSERT fails

```
query, err := sqlparser.Parse(`INSERT INTO`)

table name cannot be empty
```

### Example: INSERT with no rows to insert fails

```
query, err := sqlparser.Parse(`INSERT INTO 'a'`)

at INSERT INTO: need at least one row to insert
```

### Example: INSERT with incomplete value section fails

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (`)

at INSERT INTO: need at least one row to insert
```

### Example: INSERT with incomplete value section fails #2

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b`)

at INSERT INTO: need at least one row to insert
```

### Example: INSERT with incomplete value section fails #3

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b)`)

at INSERT INTO: need at least one row to insert
```

### Example: INSERT with incomplete value section fails #4

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b) VALUES`)

at INSERT INTO: need at least one row to insert
```

### Example: INSERT with incomplete row fails

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (b) VALUES (`)

at INSERT INTO: value count doesn't match field count
```

### Example: INSERT * fails

```
query, err := sqlparser.Parse(`INSERT INTO 'a' (*) VALUES ('1')`)

at INSERT INTO: expected at least one field to insert
```


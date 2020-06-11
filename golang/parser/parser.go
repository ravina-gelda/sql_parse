package parser
import (
	"fmt"
	"regexp"
	"strings"
    "query"
    "Token"
   // "log"
   // "runtime"

)
// https://stackoverflow.com/questions/24487943/invoking-struct-function-gives-cannot-refer-to-unexported-field-or-method
//https://golang.org/pkg/go/types/#Array

type Parser struct {	
	sql             string
	step            step
	query           query.Query
	err             error
	nextUpdateField string
}
func Parse(sql string) (query.Query, error) {
	qs, err := (&Parser{ strings.TrimSpace(sql), stepType, query.Query{}, nil, ""}).parse()
	//if (query.Query{}) == qs  {
	//	return query.Query{}, err
	//}
	return qs, err
}

func (p *Parser) parse() (query.Query, error) {
     
	q, err := p.doParse()
	p.err = err
	if p.err == nil {
		p.err = p.validate()
	}
	p.logError()
	return q, p.err
}


var reservedWords = []string{
	"(", ")", ">=", "<=", "!=", ",", "=", ">", "<", "SELECT", "INSERT INTO", "VALUES", "UPDATE", "DELETE FROM",
	"WHERE", "FROM", "SET",
}

type step int

const (
	stepType step = iota
	stepSelectField
	stepSelectFrom
	stepSelectComma
	stepSelectFromTable
	stepInsertTable
	stepInsertFieldsOpeningParens
	stepInsertFields
	stepInsertFieldsCommaOrClosingParens
	stepInsertValuesOpeningParens
	stepInsertValuesRWord
	stepInsertValues
	stepInsertValuesCommaOrClosingParens
	stepInsertValuesCommaBeforeOpeningParens
	stepUpdateTable
	stepUpdateSet
	stepUpdateField
	stepUpdateEquals
	stepUpdateValue
	stepUpdateComma
	stepDeleteFromTable
	stepWhere
	stepWhereField
	stepWhereOperator
	stepWhereValue
	stepWhereAnd
)


func (p *Parser) doParse() (query.Query, error) {
    
    token := &Token.Tokenizer{0, strings.TrimSpace(p.sql)}
	for {
		if token.I >= len(p.sql) {
			return p.query, p.err
           
		}
       
		switch p.step {
		case stepType:
			switch strings.ToUpper(token.Peek()) {
			case "SELECT":
				p.query.Type = query.Select
                //fmt.Println(p.i)
				token.Pop()
                //fmt.Println(p.i)
				p.step = stepSelectField
			case "INSERT INTO":
				p.query.Type = query.Insert
				token.Pop()
				p.step = stepInsertTable
			case "UPDATE":
				p.query.Type = query.Update
				p.query.Updates = map[string]string{}
				token.Pop()
				p.step = stepUpdateTable
			case "DELETE FROM":
				p.query.Type = query.Delete
				token.Pop()
				p.step = stepDeleteFromTable
			default:
				return p.query, fmt.Errorf("invalid query type")
			}
		case stepSelectField:
			identifier := token.Peek()
			if !isIdentifierOrAsterisk(identifier) {
				return p.query, fmt.Errorf("at SELECT: expected field to SELECT")
			}
			p.query.Fields = append(p.query.Fields, identifier)
			token.Pop()
			maybeFrom := token.Peek()
			if strings.ToUpper(maybeFrom) == "FROM" {
				p.step = stepSelectFrom
				continue
			}
			p.step = stepSelectComma
		case stepSelectComma:
			commaRWord := token.Peek()
			if commaRWord != "," {
				return p.query, fmt.Errorf("at SELECT: expected comma or FROM")
			}
			token.Pop()
			p.step = stepSelectField
		case stepSelectFrom:
			fromRWord := token.Peek()
			if strings.ToUpper(fromRWord) != "FROM" {
				return p.query, fmt.Errorf("at SELECT: expected FROM")
			}
			token.Pop()
			p.step = stepSelectFromTable
		case stepSelectFromTable:
            //fmt.Println("tttttt")
            //fmt.Println(p.i)
            //fmt.Println(p.peek())
			tableName := token.Peek()
			if len(tableName) == 0 {
				return p.query, fmt.Errorf("at SELECT: expected quoted table name")
			}
			p.query.TableName = tableName
			token.Pop()
			p.step = stepWhere
		case stepInsertTable:
			tableName := token.Peek()
			if len(tableName) == 0 {
				return p.query, fmt.Errorf("at INSERT INTO: expected quoted table name")
			}
			p.query.TableName = tableName
			token.Pop()
			p.step = stepInsertFieldsOpeningParens
		case stepDeleteFromTable:
			tableName := token.Peek()
			if len(tableName) == 0 {
				return p.query, fmt.Errorf("at DELETE FROM: expected quoted table name")
			}
			p.query.TableName = tableName
			token.Pop()
			p.step = stepWhere
		case stepUpdateTable:
			tableName := token.Peek()
			if len(tableName) == 0 {
				return p.query, fmt.Errorf("at UPDATE: expected quoted table name")
			}
			p.query.TableName = tableName
			token.Pop()
			p.step = stepUpdateSet
		case stepUpdateSet:
			setRWord := token.Peek()
			if setRWord != "SET" {
				return p.query, fmt.Errorf("at UPDATE: expected 'SET'")
			}
			token.Pop()
			p.step = stepUpdateField
		case stepUpdateField:
			identifier := token.Peek()
			if !isIdentifier(identifier) {
				return p.query, fmt.Errorf("at UPDATE: expected at least one field to update")
			}
			p.nextUpdateField = identifier
			token.Pop()
			p.step = stepUpdateEquals
		case stepUpdateEquals:
			equalsRWord := token.Peek()
			if equalsRWord != "=" {
				return p.query, fmt.Errorf("at UPDATE: expected '='")
			}
			token.Pop()
			p.step = stepUpdateValue
		case stepUpdateValue:
            //fmt.Println(quotedValue)
            //quotedValue, ln := peekQuoted(p.sql,p.sql_index)
            quotedValue := token.Peek()
            ln :=len(quotedValue)
            //fmt.Println(quotedValue)
			if ln == 0 {
				return p.query, fmt.Errorf("at UPDATE: expected quoted value")
			}
			p.query.Updates[p.nextUpdateField] = quotedValue
			p.nextUpdateField = ""
			token.Pop()
			maybeWhere := token.Peek()
			if strings.ToUpper(maybeWhere) == "WHERE" {
				p.step = stepWhere
				continue
			}
			p.step = stepUpdateComma
		case stepUpdateComma:
			commaRWord := token.Peek()
			if commaRWord != "," {
				return p.query, fmt.Errorf("at UPDATE: expected ','")
			}
			token.Pop()
			p.step = stepUpdateField
		case stepWhere:
			whereRWord := token.Peek()
			if strings.ToUpper(whereRWord) != "WHERE" {
				return p.query, fmt.Errorf("expected WHERE")
			}
			token.Pop()
			p.step = stepWhereField
		case stepWhereField:
			identifier := token.Peek()
			if !isIdentifier(identifier) {
				return p.query, fmt.Errorf("at WHERE: expected field")
			}
			p.query.Conditions = append(p.query.Conditions, query.Condition{Operand1: identifier, Operand1IsField: true})
			token.Pop()
			p.step = stepWhereOperator
		case stepWhereOperator:
			operator := token.Peek()
			currentCondition := p.query.Conditions[len(p.query.Conditions)-1]
			switch operator {
			case "=":
				currentCondition.Operator = query.Eq
			case ">":
				currentCondition.Operator = query.Gt
			case ">=":
				currentCondition.Operator = query.Gte
			case "<":
				currentCondition.Operator = query.Lt
			case "<=":
				currentCondition.Operator = query.Lte
			case "!=":
				currentCondition.Operator = query.Ne
			default:
				return p.query, fmt.Errorf("at WHERE: unknown operator")
			}
			p.query.Conditions[len(p.query.Conditions)-1] = currentCondition
			token.Pop()
			p.step = stepWhereValue
		case stepWhereValue:
			//quotedValue, ln := peekQuoted(p.sql,p.sql_index)
             quotedValue := token.Peek()
            ln :=len(quotedValue)
            //fmt.Println(ln)
            //fmt.Println(p.sql)
            
            //fmt.Println(p.sql_index)
			if ln == 0 {
				return p.query, fmt.Errorf("at WHERE: expected quoted value")
			}
			currentCondition := p.query.Conditions[len(p.query.Conditions)-1]
			currentCondition.Operand2 = quotedValue
			currentCondition.Operand2IsField = false
			p.query.Conditions[len(p.query.Conditions)-1] = currentCondition
			token.Pop()
			p.step = stepWhereAnd
		case stepWhereAnd:
			andRWord := token.Peek()
			if strings.ToUpper(andRWord) != "AND" {
				return p.query, fmt.Errorf("expected AND")
			}
			token.Pop()
			p.step = stepWhereField
		case stepInsertFieldsOpeningParens:
			openingParens := token.Peek()
			if len(openingParens) != 1 || openingParens != "(" {
				return p.query, fmt.Errorf("at INSERT INTO: expected opening parens")
			}
			token.Pop()
			p.step = stepInsertFields
		case stepInsertFields:
			identifier := token.Peek()
			if !isIdentifier(identifier) {
				return p.query, fmt.Errorf("at INSERT INTO: expected at least one field to insert")
			}
			p.query.Fields = append(p.query.Fields, identifier)
			token.Pop()
			p.step = stepInsertFieldsCommaOrClosingParens
		case stepInsertFieldsCommaOrClosingParens:
			commaOrClosingParens := token.Peek()
			if commaOrClosingParens != "," && commaOrClosingParens != ")" {
				return p.query, fmt.Errorf("at INSERT INTO: expected comma or closing parens")
			}
			token.Pop()
			if commaOrClosingParens == "," {
				p.step = stepInsertFields
				continue
			}
			p.step = stepInsertValuesRWord
		case stepInsertValuesRWord:
			valuesRWord := token.Peek()
			if strings.ToUpper(valuesRWord) != "VALUES" {
				return p.query, fmt.Errorf("at INSERT INTO: expected 'VALUES'")
			}
			token.Pop()
			p.step = stepInsertValuesOpeningParens
		case stepInsertValuesOpeningParens:
			openingParens := token.Peek()
			if openingParens != "(" {
				return p.query, fmt.Errorf("at INSERT INTO: expected opening parens")
			}
			p.query.Inserts = append(p.query.Inserts, []string{})
			token.Pop()
			p.step = stepInsertValues
		case stepInsertValues:
			//quotedValue, ln := peekQuoted(p.sql,p.sql_index)
             quotedValue := token.Peek()
            ln :=len(quotedValue)
			if ln == 0 {
				return p.query, fmt.Errorf("at INSERT INTO: expected quoted value")
			}
			p.query.Inserts[len(p.query.Inserts)-1] = append(p.query.Inserts[len(p.query.Inserts)-1], quotedValue)
			token.Pop()
			p.step = stepInsertValuesCommaOrClosingParens
		case stepInsertValuesCommaOrClosingParens:
			commaOrClosingParens := token.Peek()
			if commaOrClosingParens != "," && commaOrClosingParens != ")" {
				return p.query, fmt.Errorf("at INSERT INTO: expected comma or closing parens")
			}
			token.Pop()
			if commaOrClosingParens == "," {
				p.step = stepInsertValues
				continue
			}
			currentInsertRow := p.query.Inserts[len(p.query.Inserts)-1]
			if len(currentInsertRow) < len(p.query.Fields) {
				return p.query, fmt.Errorf("at INSERT INTO: value count doesn't match field count")
			}
			p.step = stepInsertValuesCommaBeforeOpeningParens
		case stepInsertValuesCommaBeforeOpeningParens:
			commaRWord := token.Peek()
			if strings.ToUpper(commaRWord) != "," {
				return p.query, fmt.Errorf("at INSERT INTO: expected comma")
			}
			token.Pop()
			p.step = stepInsertValuesOpeningParens
		}
	}
}

func (p *Parser) validate() error {
	if len(p.query.Conditions) == 0 && p.step == stepWhereField {
		return fmt.Errorf("at WHERE: empty WHERE clause")
	}
	if p.query.Type == query.UnknownType {
		return fmt.Errorf("query type cannot be empty")
	}
	if p.query.TableName == "" {
		return fmt.Errorf("table name cannot be empty")
	}
	if len(p.query.Conditions) == 0 && (p.query.Type == query.Update || p.query.Type == query.Delete) {
		return fmt.Errorf("at WHERE: WHERE clause is mandatory for UPDATE & DELETE")
	}
	for _, c := range p.query.Conditions {
		if c.Operator == query.UnknownOperator {
			return fmt.Errorf("at WHERE: condition without operator")
		}
		if c.Operand1 == "" && c.Operand1IsField {
			return fmt.Errorf("at WHERE: condition with empty left side operand")
		}
		if c.Operand2 == "" && c.Operand2IsField {
			return fmt.Errorf("at WHERE: condition with empty right side operand")
		}
	}
	if p.query.Type == query.Insert && len(p.query.Inserts) == 0 {
		return fmt.Errorf("at INSERT INTO: need at least one row to insert")
	}
	if p.query.Type == query.Insert {
		for _, i := range p.query.Inserts {
			if len(i) != len(p.query.Fields) {
				return fmt.Errorf("at INSERT INTO: value count doesn't match field count")
			}
		}
	}
	return nil
}


func (p *Parser) logError() {
	if p.err == nil {
		return
	}
	fmt.Println(p.sql)
	//fmt.Println(strings.Repeat(" ", p.i) + "^")
	fmt.Println(p.err)
}

func isIdentifier(s string) bool {
	for _, rw := range reservedWords {
		if strings.ToUpper(s) == rw {
			return false
		}
	}
	matched, _ := regexp.MatchString("[a-zA-Z_][a-zA-Z_0-9]*", s)
	return matched
}

func isIdentifierOrAsterisk(s string) bool {
	return isIdentifier(s) || s == "*"
}


//func PrintMemUsage() {
  //      var m runtime.MemStats
    //    runtime.ReadMemStats(&m)
        // For info on each, see: https://golang.org/pkg/runtime/#MemStats
      //  fmt.Printf("Alloc = %v MiB", bToMb(m.Alloc))
        //fmt.Printf("\tTotalAlloc = %v MiB", bToMb(m.TotalAlloc))
        //fmt.Printf("\tSys = %v MiB", bToMb(m.Sys))
        //fmt.Printf("\tNumGC = %v\n", m.NumGC)
//}

//func bToMb(b uint64) uint64 {
  //  return b / 1024 / 1024
//}


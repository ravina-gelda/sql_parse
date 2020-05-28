package main

import (
   // "os"
	"fmt"
	"regexp"
	"strings"
    //"query"

   // "log"
)
type tokenizer struct {
	i               int
	sql             string
	//step            step
	//query           query.Query
	err             error
	//nextUpdateField string
}
var reservedWords = []string{
	"(", ")", ">=", "<=", "!=", ",", "=", ">", "<", "SELECT", "INSERT INTO", "VALUES", "UPDATE", "DELETE FROM",
	"WHERE", "FROM", "SET",
}
func (p *tokenizer) peek() string {
	peeked, _ := p.peekWithLength()
	return peeked
}

func (p *tokenizer) pop() []string {
    fmt.Println(p.sql)
    var newarr []string
    for{
    if p.i >= len(p.sql) {
			return newarr
		}
	peeked, len := p.peekWithLength()
        fmt.Println(peeked)
	p.i += len
	p.popWhitespace()
        newarr=append(newarr,peeked)
	//return peeked
}

    return newarr
}


func (p *tokenizer) popWhitespace() {
	for ; p.i < len(p.sql) && p.sql[p.i] == ' '; p.i++ {
	}
}

func (p *tokenizer) peekWithLength() (string, int) {
	if p.i >= len(p.sql) {
		return "", 0
	}
	for _, rWord := range reservedWords {
		token := strings.ToUpper(p.sql[p.i:min(len(p.sql), p.i+len(rWord))])
		if token == rWord {
			return token, len(token)
		}
	}
	if p.sql[p.i] == '\'' { // Quoted string
		return p.peekQuotedStringWithLength()
	}
	return p.peekIdentifierWithLength()
}

func (p *tokenizer) peekQuotedStringWithLength() (string, int) {
	if len(p.sql) < p.i || p.sql[p.i] != '\'' {
		return "", 0
	}
	for i := p.i + 1; i < len(p.sql); i++ {
		if p.sql[i] == '\'' && p.sql[i-1] != '\\' {
			return p.sql[p.i+1 : i], len(p.sql[p.i+1:i]) + 2 // +2 for the two quotes
		}
	}
	return "", 0
}

func (p *tokenizer) peekIdentifierWithLength() (string, int) {
	for i := p.i; i < len(p.sql); i++ {
		if matched, _ := regexp.MatchString(`[a-zA-Z0-9_*]`, string(p.sql[i])); !matched {
			return p.sql[p.i:i], len(p.sql[p.i:i])
		}
	}
	return p.sql[p.i:], len(p.sql[p.i:])
}
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
func main() {
    var sql string ="SELECT a FROM 'b'"
    fmt.Println(strings.TrimSpace(sql))
//parser{0, strings.TrimSpace(sql), stepType, query.Query{}, nil, ""}
    // var newarr []string
    var newarr []string = (&tokenizer{0, strings.TrimSpace(sql), nil}).pop()
    fmt.Printf("%v", newarr)
}

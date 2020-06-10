package Token

import (
   // "os"
	//"fmt"
	"regexp"
	"strings"
    //"query"

   // "log"
)
type Tokenizer struct {
	I               int
	Sql             string
	//step            step
	//query           query.Query
	//Err             error
	//nextUpdateField string
}
var identifiers = []string{
	"(", ")", ">=", "<=", "!=", ",", "=", ">", "<", "SELECT", "INSERT INTO", "VALUES", "UPDATE", "DELETE FROM",
	"WHERE", "FROM", "SET",
}
func (p *Tokenizer) Peek() string {
	peeked, _ := p.peekWithLength()
	return peeked
}

func (p *Tokenizer) Pop() string {
	peeked, len := p.peekWithLength()
	p.I += len
	p.popWhitespace()
	return peeked
}

func (p *Tokenizer) popWhitespace() {
	for ; p.I < len(p.Sql) && p.Sql[p.I] == ' '; p.I++ {
	}
}

func (p *Tokenizer) peekWithLength() (string, int) {
	if p.I >= len(p.Sql) {
		return "", 0
	}
	for _, rWord := range identifiers {
		token := strings.ToUpper(p.Sql[p.I:min(len(p.Sql), p.I+len(rWord))])
		if token == rWord {
			return token, len(token)
		}
	}
	if p.Sql[p.I] == '\'' { // Quoted string
		return p.PeekQuotedStringWithLength()
	}
	return p.peekIdentifierWithLength()
}

func (p *Tokenizer) PeekQuotedStringWithLength() (string, int) {
	if len(p.Sql) < p.I || p.Sql[p.I] != '\'' {
		return "", 0
	}
	for i := p.I + 1; i < len(p.Sql); i++ {
		if p.Sql[i] == '\'' && p.Sql[i-1] != '\\' {
			return p.Sql[p.I+1 : i], len(p.Sql[p.I+1:i]) + 2 // +2 for the two quotes
		}
	}
	return "", 0
}

func (p *Tokenizer) peekIdentifierWithLength() (string, int) {
	for i := p.I; i < len(p.Sql); i++ {
		if matched, _ := regexp.MatchString(`[a-zA-Z0-9_*]`, string(p.Sql[i])); !matched {
			return p.Sql[p.I:i], len(p.Sql[p.I:i])
		}
	}
	return p.Sql[p.I:], len(p.Sql[p.I:])
}
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}


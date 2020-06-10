use std::str::Chars;
use std::iter::Peekable;
pub struct TokenizerError(String);

pub enum Token{
    word,
    space,
    semicolon,    
}
#[derive(PartialEq, Eq)]

pub enum Step {
    stepType,
    selectField,
    validateSelectFromOrComma,
    selectFromTable,
    insertTable,
    insertFieldsOpeningParens,
    insertFieldsCommaOrClosingParens,
    insertFields,
    insertValuesRWord,
    insertValues,
    insertValuesOpeningParens,
    insertValuesCommaOrClosingParens,
    insertValuesCommaBeforeOpeningParens,
    updateTable,
    updateSet,
    updateField,
    updateEquals,
    updateValue,
    deleteFromTable,
    stepWhere,
    whereField,
    whereOperator,
    whereValue,
    whereAnd,
    validateUpdateCommaOrWhere,
}
pub struct Tokenizer {
    pub reserved_keywords : Vec<String>,
}
impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer{
            reserved_keywords : vec![String::from("("),
            String::from( ")")
            ,String::from( ">=")
            ,String::from( "<=")
            ,String::from( "!=")
            ,String::from( ",")
            ,String::from( "=")
            ,String::from(">")
            ,String::from( "<")
            ,String::from( "SELECT")
            ,String::from( "INSERT INTO")
            ,String::from( "VALUES")
            ,String::from( "UPDATE")
            ,String::from("DELETE FROM")
            ,String::from( "WHERE")
            ,String::from( "FROM")
            ,String::from( "SET")]
        }
    }
    pub fn get_tokens(&self, query: String) -> Result<Vec<String>, TokenizerError> {
        let mut tokens = Vec::new();
        let mut peekable = query.trim().chars().peekable();
        while let Some(token) = self.next_token(&mut peekable)? {
            tokens.push(token);
        }
        Ok(tokens)
        
    }
    fn next_token(&self,chars: &mut Peekable<Chars<'_>>) -> Result<Option<String>,TokenizerError> {
        let s = String::new();
        let (t , present)  = self.get_token_if_present(chars);
        if present {
            let mut c = 0;
            while c < t.len(){
                chars.next();
                c = c+1;
            }
            self.remove_spaces(chars);
            return Ok(Some(t))
        }
        match chars.peek() {
            Some(&ch)  => match ch {
                '\'' => self.get_quoted_string(chars),
                other => self.get_next_string(chars)
                }
            None => Ok(None),
        }
        
    }
    fn get_token_if_present(&self, chars: &mut Peekable<Chars<'_>>) -> (String , bool) {
        let absent = String::new();
        for token in self.reserved_keywords.iter(){
            let token_str = String::from(token);
            let count = 0;
            let mut copy = chars.clone();
            let mut s = String::new();
            let mut c = 0;
            while let Some(&ch) = copy.peek()  {
                c+=1;
                copy.next();
                s.push(ch);
                if c >= token_str.len(){
                    break;
                }
            }
            if s.eq_ignore_ascii_case(&token_str){
                return (token_str, true)
            }
        
        }
        return (absent, false)
    }
    fn get_quoted_string(&self,chars:&mut Peekable<Chars<'_>>)->Result<Option<String>,TokenizerError>{
        let mut s = String::new();
        chars.next(); // consume the opening quote
        while let Some(&ch) = chars.peek() {
            match ch {
                '\'' => {
                    chars.next(); // consume
                    let escaped_quote = chars.peek().map(|c| *c == '\'').unwrap_or(false);
                    if escaped_quote {
                        s.push('\'');
                        chars.next();
                    } else {
                        break;
                    }
                }
                _ => {
                    chars.next(); // consume
                    s.push(ch);
                }
            }
        }
        self.remove_spaces(chars);
        Ok(Some(s))
    }
    fn remove_spaces(&self,chars:&mut Peekable<Chars<'_>>) {
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\n' | '\t'  => {
                    chars.next(); //consume
                }
                _ => {
                    break;
                } 
            }
        }
    }
    fn get_next_string(&self,chars:&mut Peekable<Chars<'_>>)->Result<Option<String>,TokenizerError> {
        let mut s = String::new();
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\n' | '\t'   => {
                    chars.next(); //consume
                    break;
                }
                ',' | ')' | ']'  => {
                    if s.len() >0 {
                        break;
                    }
                    else {
                        chars.next();
                        s.push(ch);
                        break;
                    }
                }
                _ => {
                    chars.next();
                    s.push(ch);
                } 
            }
        }
        self.remove_spaces(chars);
        Ok(Some(s))   
    }
    
    
    fn tokenize_single_quoted_string(&self, chars: &mut Peekable<Chars<'_>>) -> String {
        
        let mut s = String::new();
        chars.next(); // consume the opening quote
        while let Some(&ch) = chars.peek() {
            match ch {
                '\'' => {
                    chars.next(); // consume
                    let escaped_quote = chars.peek().map(|c| *c == '\'').unwrap_or(false);
                    if escaped_quote {
                        s.push('\'');
                        chars.next();
                    } else {
                        break;
                    }
                }
                _ => {
                    chars.next(); // consume
                    s.push(ch);
                }
            }
        }
        s
    }
    pub fn is_identifier_or_asterix(&self, token:&String)->bool {
        return self.is_identifier(token) || String::from("*").eq_ignore_ascii_case(&token[..]) ;
    }
    pub fn is_identifier(&self, token:&String)-> bool {
        //TODO figure out how to handle case here. 
        for u in self.reserved_keywords.iter(){
            if String::from(u).eq_ignore_ascii_case(token) {
                return false;
            }
        }
        true
            
    }
}
#[cfg(test)]
mod toenizer_test{
    use super::*; 
    #[test]
    fn test_tokenize_simple(){
    let p = Tokenizer::new();
    let t = p.get_tokens(String::from("SELECT * FROM \'A and B\'")).unwrap_or(vec![]);
    let expected = vec!["SELECT","*", "FROM", "A and B"] ;
    assert_eq![t,expected ];
    }
    #[test]
    fn test_tokenize_simple_again(){
    let p = Tokenizer::new();
    let t = p.get_tokens(String::from("SELECT a, b, c FROM \'A and B\'")).unwrap_or(vec![]);
    let expected = vec!["SELECT","a", ",","b", ",", "c", "FROM", "A and B"] ;
    assert_eq![t,expected ];
    }
    #[test]
    fn test_tokenize_simple_insert(){
    let p = Tokenizer::new();
    
    let t = p.get_tokens(String::from("Insert into b values (2, 4)")).unwrap_or(vec![]);
    let expected = vec!["INSERT INTO","b", "VALUES","(", "2", ",", "4", ")"] ;
    assert_eq![t,expected ];
    }
}

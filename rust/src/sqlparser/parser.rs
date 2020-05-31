
use super::ast;
use super::tokens;
use std::str::Chars;
use std::iter::Peekable;
pub struct TokenizerError(String);




trait Parser{
    //Trait/Interface for the parser. Given the 
    fn parse(&self, query : &str) -> (ast::Query, ast::ErrorCode);

}
struct SQLParser {
    
    pub sql : String,
    pub step: tokens::Step,
    pub query : ast::Query,
    pub error : ast::ErrorCode,
    pub reserved_keywords : Vec<String>

}
impl SQLParser {
    
    pub fn new() ->Self {
        let p = SQLParser{
            sql : String::from(""),
            step: tokens::Step::stepType,
            query: ast::Query::new(), 
            error: ast::ErrorCode::None,
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
        };
        p
    } 
    pub fn get_tokens(&self, query: String) -> Result<Vec<String>, TokenizerError> {
        let mut tokens = Vec::new();
        let mut peekable = query.chars().peekable();
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
    fn is_identifier_or_asterix(&self, token:&String)->bool {
        return self.is_identifier(token) || String::from("*").eq_ignore_ascii_case(&token[..]) ;
    }
    fn is_identifier(&self, token:&String)-> bool {
        //TODO figure out how to handle case here. 
        for u in self.reserved_keywords.iter(){
            if String::from(u).eq_ignore_ascii_case(token) {
                return true;
            }
        }
        false
            

    }

}
impl Parser for SQLParser {
    fn parse(&self, query: &str) -> (ast::Query, ast:: ErrorCode) {
        let mut q = ast::Query::new();
        let tokens = self.get_tokens(String::from(query));
        let mut updateNextField : &str = ""; 
        if !tokens.is_ok() {
            return (q, ast::ErrorCode::TokenizationError)

        } 
        let  mut step: tokens::Step = tokens::Step::stepType;

        for token in tokens.unwrap_or(vec![]).iter().peekable(){
            match step {
                tokens::Step::stepType => {
                    if String::from("SELECT").eq_ignore_ascii_case(&token[..]) {
                        // we received a select as the first state . WE change the query type to select and proceed to the next state. 
                        q.query_type = ast::QueryType::SELECT;
                        step = tokens::Step::selectField;
                    }
                    else if String::from("INSERT INTO").eq_ignore_ascii_case(&token[..]) {
                        // here we validate whether the next token is indeed from 
                        q.query_type = ast::QueryType::INSERT;
                        step = tokens::Step::insertTable;
                    }
                    else if String::from("DELETE FROM").eq_ignore_ascii_case(&token[..]) {
                        q.query_type = ast::QueryType::DELETE;
                        step = tokens::Step::deleteFromTable;
                    }
                    else if String::from("UPDATE").eq_ignore_ascii_case(&token[..]) {
                        q.query_type = ast::QueryType::UPDATE;
                        step = tokens::Step::deleteFromTable;
                    
                        //TODO: add metadata for a select query to the query struct

                    }
                    else {
                        return (q,ast::ErrorCode::ParseError)
                    }

                    
                }
                tokens::Step::selectField => {
                    if !self.is_identifier_or_asterix(token){
                        return (q,ast::ErrorCode::ParseError);
                    }
                    q.fields.push(String::from(&token[..]));
                    step = tokens::Step::validateSelectFromOrComma;



                }
                tokens::Step::validateSelectFromOrComma => {
                    if String::from(",").eq_ignore_ascii_case(&token[..]){
                        step = tokens::Step::selectField;

                    }
                    else if String::from("FROM").eq_ignore_ascii_case(&token[..]) {
                        step = tokens::Step::selectFromTable;
                        continue;
                    
                    }
                    else {
                        //TODO: return proper error
                        return (q, ast::ErrorCode::ParseError)
                    }
                
                }
                tokens::Step::selectFromTable => {
                    if token.len() == 0{
                        //TODO: return proper error
                        return (q, ast::ErrorCode::ParseError);

                    }
                    q.table = String::from(token);
                    step = tokens::Step::stepWhere;
                }
                tokens::Step::insertTable => {
                    if token.len() == 0 {
                        //TODO: throw  proper error
                        return (q, ast::ErrorCode::ParseError);
                    }
                    q.table = String::from(token);
                    step = tokens::Step::insertFieldsOpeningParens;
                }
                tokens::Step::deleteFromTable =>{
                    if token.len() == 0 {
                        //TODO: throw  proper error
                        return (q, ast::ErrorCode::ParseError);
                    }
                    q.table = String::from(token);
                    step = tokens::Step::whereField;

                }
                tokens::Step::updateTable => {
                    if token.len() == 0 {
                     //TODO: throw  proper error
                     return (q, ast::ErrorCode::ParseError);
                    }
                    q.table = String::from(token);
                    step = tokens::Step::updateSet;
                }
                tokens::Step::updateSet => {
                    if ! String::from("SET").eq_ignore_ascii_case(&token[..]) {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    step = tokens::Step::updateField;
                }
                tokens::Step::updateField =>{
                    if ! self.is_identifier(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    updateNextField = &token[..];
                    step = tokens::Step::updateEquals;

                }
                tokens::Step::updateEquals => {
                    if ! String::from("=").eq_ignore_ascii_case(&token[..]){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    step = tokens::Step::updateValue;
                }
                
                tokens::Step::updateValue =>{
                    if token.len() == 0 {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    //TODO: add update fields 
                    q.update_fields.insert(String::from(updateNextField),String::from(token));
                    updateNextField = "";
                    step = tokens::Step::validateUpdateCommaOrWhere;
                }
                tokens::Step::validateUpdateCommaOrWhere => {
                    if String::from("WHERE").eq_ignore_ascii_case(&token[..]){
                        step = tokens::Step::whereField;
                        continue;
                    }
                    else if String::from(",").eq_ignore_ascii_case(&token[..]){
                        step = tokens::Step::updateField;
                        continue;
                    }
                    else {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);

                    }
                }
                tokens::Step::stepWhere  => {
                    if !String::from("WHERE").eq_ignore_ascii_case(&token[..]){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    step = tokens::Step::whereField;
                }
                tokens::Step::whereField =>{
                    if !self.is_identifier(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    let mut c = ast::Condition::new();
                    c.operand1 = String::from(&token[..]);
                    q.conditions.push(c);
                    step = tokens::Step::whereOperator;
                }
                tokens::Step::whereOperator =>{
                    let mut c = q.conditions.pop().unwrap();
                    if String::from("=").eq_ignore_ascii_case(&token[..]) {
                        c.operator = ast::Operator::eq;
                    }
                    else if String::from(">").eq_ignore_ascii_case(&token[..]){
                        c.operator = ast::Operator::gt;
                    }
                    else if String::from(">=").eq_ignore_ascii_case(&token[..]){
                        c.operator = ast::Operator::gte;

                    }
                    else if String::from("<").eq_ignore_ascii_case(&token[..]){
                        c.operator = ast::Operator::lt;
                    }
                    else if String::from("<=").eq_ignore_ascii_case(&token[..]){
                        c.operator = ast::Operator::lte;

                    }
                    else if String::from("!=").eq_ignore_ascii_case(&token[..]){
                        c.operator = ast::Operator::ne;

                    }
                    else {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    q.conditions.push(c);
                    step = tokens::Step::whereValue;
                }
                tokens::Step::whereValue  => {
                    if token.len() == 0 {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    let mut c = q.conditions.pop().unwrap();
                    c.operand2 = String::from(&token[..]);
                    c.operand_2_is_field = false;
                    step = tokens::Step::whereAnd;

                }
                tokens::Step::whereAnd =>{
                    if  ! String::from("AND").eq_ignore_ascii_case("AND"){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    
                    }
                    step = tokens::Step::whereField;
                }
                tokens::Step::insertFieldsOpeningParens =>{
                    if token.len() != 1 && String::from("(").eq_ignore_ascii_case(&token[..]) {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    step = tokens::Step::insertFields;
                }
                tokens::Step::insertFields =>{
                    if ! self.is_identifier(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    q.fields.push(String::from(token));
                    step = tokens::Step::insertFieldsCommaOrClosingParens;

                }
                tokens::Step::insertFieldsCommaOrClosingParens =>{
                    if token != "," && token != ")" {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    if token == ","{
                        step = tokens::Step::insertFields;
                        continue;
                    }
                    step = tokens::Step::insertValuesRWord;
                }
                tokens::Step::insertValuesRWord =>{
                    if ! String::from("VALUES").eq_ignore_ascii_case(&token[..]){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);

                    }
                    step = tokens::Step::insertValuesOpeningParens;
                }
                tokens::Step::insertValuesOpeningParens =>{
                    if token != "(" {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    q.insert_fields.push(Vec::new());
                    step = tokens::Step::insertValues;
                }
                tokens::Step::insertValues =>{
                    if token.len() == 0 {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);
                    }
                    let mut lval = q.insert_fields.pop().unwrap();
                    lval.push(String::from(token));
                    q.insert_fields.push(lval);
                    step = tokens::Step::insertValuesCommaOrClosingParens;
                }
                tokens::Step::insertValuesCommaOrClosingParens =>{
                    if token != "," && token != ")" {
                         //TODO: throw proper error code
                         return (q, ast::ErrorCode::ParseError);
                    }
                    if token == "," {
                        step = tokens::Step::insertValues;
                        continue;
                    }
                    let last = q.insert_fields.pop().unwrap();
                    if last.len() < q.fields.len(){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);                        
                    }

                    step = tokens::Step::insertValuesCommaBeforeOpeningParens;
                }
                tokens::Step::insertValuesCommaBeforeOpeningParens =>{
                    if token != "," {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError);   
                    }
                    step = tokens::Step::insertValuesOpeningParens;
                }
                
            }
            

        }

        return (q, ast::ErrorCode::None)
        

    }
    
    
}
impl  SQLParser{
    
}
#[cfg(test)]
mod parser_test{
    use super::*;
    fn test_simple_parse(){
        let query = "SELECT a, b FROM 'b'";
        let p = SQLParser::new();
        let (a , error) = p.parse(query);


        assert_eq!(a.table, "b");
        assert_eq!(a.fields, vec!["a", "b"]);
        assert!(error == ast::ErrorCode::None);
        assert!(String::from(query).starts_with("SELECT a"));
    }
    #[test]
    fn test_tokenize_simple(){
    let p = SQLParser::new();
    let t = p.get_tokens(String::from("SELECT * FROM \'A and B\'")).unwrap_or(vec![]);
    let expected = vec!["SELECT","*", "FROM", "A and B"] ;

    assert_eq![t,expected ];
    }
    #[test]
    fn test_tokenize_simple_again(){
    let p = SQLParser::new();
    let t = p.get_tokens(String::from("SELECT a, b, c FROM \'A and B\'")).unwrap_or(vec![]);
    let expected = vec!["SELECT","a", ",","b", ",", "c", "FROM", "A and B"] ;

    assert_eq![t,expected ];
    }
    #[test]
    fn test_tokenize_simple_insert(){
    let p = SQLParser::new();
    
    let t = p.get_tokens(String::from("Insert into b values (2, 4)")).unwrap_or(vec![]);
    let expected = vec!["INSERT INTO","b", "VALUES","(", "2", ",", "4", ")"] ;

    assert_eq![t,expected ];
    }

}

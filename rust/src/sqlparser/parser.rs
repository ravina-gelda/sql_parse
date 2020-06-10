
use super::ast;
use super::tokens;
pub trait Parser{
    //Trait/Interface for the parser. Given the 
    fn parse(&self, query : &str) -> (ast::Query, ast::ErrorCode);
}
pub struct SQLParser {
    
    pub sql : String,
    pub step: tokens::Step,
    pub query : ast::Query,
    pub error : ast::ErrorCode,
}
impl SQLParser {
    
    pub fn new() ->Self {
        let p = SQLParser{
            sql : String::from(""),
            step: tokens::Step::stepType,
            query: ast::Query::new(), 
            error: ast::ErrorCode::None,
            
        };
        p
    } 
    
}
impl Parser for SQLParser {
    fn parse(&self, query: &str) -> (ast::Query, ast:: ErrorCode) {
        let tokenizer = tokens::Tokenizer::new();
        let mut q = ast::Query::new();
        let tokens = tokenizer.get_tokens(String::from(query));
        let mut updateNextField : &str = ""; 
        if !tokens.is_ok() {
            return (q, ast::ErrorCode::TokenizationError(String::from("Unable to parse input query")))
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
                        step = tokens::Step::updateTable;
                    
                        //TODO: add metadata for a select query to the query struct

                    }
                    else {
                        return (q,ast::ErrorCode::ParseError(String::from("Invalid query type")))

                    
                    }
                    
                }
                tokens::Step::selectField => {
                    if !tokenizer.is_identifier_or_asterix(token){
                        return (q,ast::ErrorCode::ParseError
                        (String::from("at SELECT: expected field to select")));
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
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at SELECT expected FROM or a comma after the field")));
                    }
                
                }
                tokens::Step::selectFromTable => {
                    if token.len() == 0{
                        //TODO: return proper error
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at SELECT: expected quoted table name")
                        )
                        );
                    }
                    q.table = String::from(token);
                    step = tokens::Step::stepWhere;
                }
                tokens::Step::insertTable => {
                    if token.len() == 0 {
                        //TODO: throw  proper error
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected quoted table name")
                        ));
                    }
                    q.table = String::from(token);
                    step = tokens::Step::insertFieldsOpeningParens;
                }
                tokens::Step::deleteFromTable =>{
                    if token.len() == 0 {
                        //TODO: throw  proper error
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at DELETE FROM: expected quoted table name")
                        ));
                    }
                    q.table = String::from(token);
                    step = tokens::Step::stepWhere;
                }
                tokens::Step::updateTable => {
                    if token.len() == 0 {
                     //TODO: throw  proper error
                     return (q, ast::ErrorCode::ParseError(
                         String::from("at UPDATE: expected quoted table name")
                     )
                    );
                    }
                    q.table = String::from(token);
                    step = tokens::Step::updateSet;
                }
                tokens::Step::updateSet => {
                    if ! String::from("SET").eq_ignore_ascii_case(&token[..]) {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at UPDATE: expected 'SET'")
                        ));
                    }
                    step = tokens::Step::updateField;
                }
                tokens::Step::updateField =>{
                    if ! tokenizer.is_identifier(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("At UPDATE: expected atleast one field to update")
                        ));
                    }
                    updateNextField = &token[..];
                    step = tokens::Step::updateEquals;
                }
                tokens::Step::updateEquals => {
                    if ! String::from("=").eq_ignore_ascii_case(&token[..]){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at UPDATE: expected '='")
                        )
                        );
                    }
                    step = tokens::Step::updateValue;
                }
                
                tokens::Step::updateValue =>{
                    if token.len() == 0 {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at UPDATE: expected quoted value")
                        )
                        );
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
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at UPDATE: expected , or WHERE")
                        )
                        );
                    }
                }
                tokens::Step::stepWhere  => {
                    if !String::from("WHERE").eq_ignore_ascii_case(&token[..]){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("expected WHERE")
                        )
                        );
                    }
                    step = tokens::Step::whereField;
                }
                tokens::Step::whereField =>{
                    if !tokenizer.is_identifier(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at WHERE: expected field")
                        )
                        );
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
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at WHERE: unknown operator")
                        ));
                    }
                    q.conditions.push(c);
                    step = tokens::Step::whereValue;
                }
                tokens::Step::whereValue  => {
                    if token.len() == 0 {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at WHERE: expected quoted value")
                        )
                        );
                    }
                    let mut c = q.conditions.pop().unwrap();
                    c.operand2 = String::from(&token[..]);
                    c.operand_2_is_field = false;
                    q.conditions.push(c);
                    step = tokens::Step::whereAnd;
                }
                tokens::Step::whereAnd =>{
                    if  ! String::from("AND").eq_ignore_ascii_case(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("expected AND")
                        ));
                    
                    }
                    step = tokens::Step::whereField;
                }
                tokens::Step::insertFieldsOpeningParens =>{
                    if token.len() != 1 && String::from("(").eq_ignore_ascii_case(&token[..]) {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERt INTO: expected (")

                        ));
                    }
                    step = tokens::Step::insertFields;
                }
                tokens::Step::insertFields =>{
                    if ! tokenizer.is_identifier(token){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected at least one field to insert")
                        )
                        );
                    }
                    q.fields.push(String::from(token));
                    step = tokens::Step::insertFieldsCommaOrClosingParens;
                }
                tokens::Step::insertFieldsCommaOrClosingParens =>{
                    if token != "," && token != ")" {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected comma or closing parens")
                        )
                        );
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
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected 'VALUES' ")
                        ));
                    }
                    step = tokens::Step::insertValuesOpeningParens;
                }
                tokens::Step::insertValuesOpeningParens =>{
                    if token != "(" {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected opening parens")
                        ));
                    }
                    q.insert_fields.push(Vec::new());
                    step = tokens::Step::insertValues;
                }
                tokens::Step::insertValues =>{
                    if token.len() == 0 {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected quoted value")
                        ));
                    }
                    let mut lval = q.insert_fields.pop().unwrap();
                    lval.push(String::from(token));
                    q.insert_fields.push(lval);
                    step = tokens::Step::insertValuesCommaOrClosingParens;
                }
                tokens::Step::insertValuesCommaOrClosingParens =>{
                    if token != "," && token != ")" {
                         //TODO: throw proper error code
                         return (q, ast::ErrorCode::ParseError(
                             String::from("at INSERT INTO: expected comma or closing parens")
                         ));
                    }
                    if token == "," {
                        step = tokens::Step::insertValues;
                        continue;
                    }
                    let last = q.insert_fields.pop().unwrap();
                    if last.len() < q.fields.len(){
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: value count does not match field count")
                        )
                        );                        
                    }
                    step = tokens::Step::insertValuesCommaBeforeOpeningParens;
                }
                tokens::Step::insertValuesCommaBeforeOpeningParens =>{
                    if token != "," {
                        //TODO: throw proper error code
                        return (q, ast::ErrorCode::ParseError(
                            String::from("at INSERT INTO: expected comma")
                    )
                        );   
                    }
                    step = tokens::Step::insertValuesOpeningParens;
                }
                
            }
            
        }
        return (q, ast::ErrorCode::None)
        
    }
    
    
}

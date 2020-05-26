
use super::ast;
use super::tokens;
trait Parser{
    //Trait/Interface for the parser. Given the 
    fn parse(&self, query : &str) -> (ast::Query, ast::ErrorCode);

}
struct SQLParser {
    pub pos : i32,
    pub sql : String,
    pub step: tokens::Step,


    

    
}
impl Parser for SQLParser {
    fn parse(&self, query: &str) -> (ast::Query, ast:: ErrorCode) {
        let q = ast::Query{
            table:String::from("table"),
            conditions: vec![],
            query_type: ast::QueryType::Insert,
            fields: vec![]
        };

         return (q, ast::ErrorCode::None)
        

    }
    
}
mod parser_test{
    use super::*;
    fn test_simple_parse(){
        let query = "SELECT a FROM 'b'";
        let p = SQLParser{
            pos: 1,
            sql : String::from(""),
            step: tokens::Step::stepType
        };
        let (a , error) = p.parse(query);


        assert_eq!(a.table, "b");
        assert_eq!(a.fields, vec!["a"]);
        assert!(error == ast::ErrorCode::None)
    }

}

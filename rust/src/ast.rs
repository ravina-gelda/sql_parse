
trait Parser{
    //Trait/Interface for the parser. Given the 
    fn parse(query : &str) -> dyn Ast;

}
trait Ast {
    fn get_type(&self);
}
//Trait for a table. A table can be a simple table which just has the name of the table, or a complex table , which is represented as an AST. 
trait Table {
    // TODO: change to give enums
    fn get_table_type(&self) -> &str;

}
struct SimpleTable {
    name: String

}
impl Table for SimpleTable {
    fn get_table_type(&self) -> &str {
       return "simple"
    }
}
impl Table for AstTable {
    fn get_table_type(&self) -> &str {
        return "ast";
    }
}
struct AstTable {
    pub ast: dyn Ast
    
}
trait Condition{
    fn get_condition_type(&self)->&str;
}

struct SimpleCondition {
    field: String,
    op: String,
    value: String

}
impl Condition for SimpleCondition{
    fn  get_condition_type(&self) -> &str {
        "simple"
    }
}

struct SelectAST<'a> {
    table: &'a (dyn Table + 'a),
    where_condition: &'a (dyn Condition+ 'a),

}
mod tests {
    use super::*;
    #[test]
    fn test_simple_table_create(){
        let table = SimpleTable{
            name: String::from( "name")
        };
        assert_eq!(table.get_table_type(), "simple");
        assert_eq!(table.name, "name");
    }
}

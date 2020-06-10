
#[cfg(test)]
use super::parser;
use super::ast;
use super::parser::Parser;
#[test]
fn test_select_simple_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "SELECT a from 'b'";
    let (query, error_code)  = p.parse(q);
    println!("{}",query.table);
    println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a")]);
}
#[test]
fn test_simple_with_lower_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a fRoM 'b'";
    let (query, error_code)  = p.parse(q);
    println!("{}",query.table);
    println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a")]);
}
#[test]
fn test_simple_many_fields_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b'";
    let (query, error_code)  = p.parse(q);
    println!("{}",query.table);
    println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
}
#[test]
fn test_simple_where_eq_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a = '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
    println!("{}",query.conditions[0].operand1);
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::eq,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }])
}
#[test]
fn test_simple_where_gt_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a > '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
    println!("{}",query.conditions[0].operand1);
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::gt,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }])
}
#[test]
fn test_simple_where_gte_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a >= '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
    println!("{}",query.conditions[0].operand1);
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::gte,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }])
}
#[test]
fn test_simple_where_lte_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a <= '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
    println!("{}",query.conditions[0].operand1);
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::lte,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }])
}
#[test]
fn test_simple_where_lt_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a < '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
    println!("{}",query.conditions[0].operand1);
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::lt,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }])
}
#[test]
fn test_simple_where_ne_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a != '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);
    println!("{}",query.conditions[0].operand1);
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::ne,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }])
}
#[test]
fn test_simple_where_and_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select a, c, d FROM 'b' where a != '1' and b = '2'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("a"), String::from("c"), String::from("d")]);  
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::ne,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    },
    ast::Condition{
        operand1: String::from("b"),
        operator:  ast::Operator::eq,
        operand2: String::from("2"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }

    
    ]) 
}
fn test_simple_where_star_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "select * FROM 'b'";
    let (query, error_code)  = p.parse(q);
    
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::SELECT);
    assert!(query.table == String::from( "b"));
    assert!(query.fields == vec![String::from("*")]);   
}
#[test]
fn test_update_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE 'a' SET b = 'hello' WHERE a = '1'";
    let (query, error_code)  = p.parse(q);
   
    let mut  h = std::collections::HashMap::new();
    h.insert(String::from("b"),String::from("hello"));
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::UPDATE);
    assert!(query.table == String::from( "a"));
    assert_eq!(query.update_fields, h );   
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::eq,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }
    ]);
}
#[test]
fn test_update_success_multiple(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE 'a' SET b = 'hello', c = 'bye' WHERE a = '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    let mut  h = std::collections::HashMap::new();
    h.insert(String::from("b"),String::from("hello"));
    h.insert(String::from("c"), String::from("bye"));
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::UPDATE);
    assert!(query.table == String::from( "a"));
    assert_eq!(query.update_fields, h );   
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::eq,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }]);
}
#[test]
fn test_update_success_multiple_and(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE 'a' SET b = 'hello', c = 'bye' WHERE a = '1' AND b = '789'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    let mut  h = std::collections::HashMap::new();
    h.insert(String::from("b"),String::from("hello"));
    h.insert(String::from("c"), String::from("bye"));
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::UPDATE);
    assert!(query.table == String::from( "a"));
    assert_eq!(query.update_fields, h );   
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("a"),
        operator:  ast::Operator::eq,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }, ast::Condition{
        operand1: String::from("b"),
        operator:  ast::Operator::eq,
        operand2: String::from("789"),
        operand_1_is_field : true,
        operand_2_is_field : false}]);
}


#[test]
fn test_delete_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "DELETE FROM 'a' WHERE b = '1'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::DELETE);
    assert!(query.table == String::from( "a"));
    assert!(query.conditions == vec![ast::Condition{
        operand1: String::from("b"),
        operator:  ast::Operator::eq,
        operand2: String::from("1"),
        operand_1_is_field : true,
        operand_2_is_field : false
    }]);
}
#[test]
fn test_insert_success(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' (b) VALUES ('1')'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::INSERT);
    assert!(query.table == String::from( "a"));
    assert!(query.fields == vec![String::from("b")]); 
    assert!(query.insert_fields == vec![vec![String::from("1")]]); 
}
#[test]
fn test_insert_success_multiple_fields(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' (b, c, d) VALUES ('1', '2', '3')'";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::INSERT);
    assert!(query.table == String::from( "a"));
    assert!(query.fields == vec![String::from("b"), String::from("c"), String::from("d")]); 
    assert!(query.insert_fields == vec![vec![String::from("1"), String::from("2"), String::from("3")]]); 
}
#[test]

fn test_insert_success_multiple_values(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' (b, c, d) VALUES ('1', '2', '3'), ('4', '5', '6' )";
    let (query, error_code)  = p.parse(q);
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
    assert!(error_code == ast::ErrorCode::None);
    assert!(query.query_type ==  ast::QueryType::INSERT);
    assert!(query.table == String::from( "a"));
    assert!(query.fields == vec![String::from("b"), String::from("c"), String::from("d")]); 
    assert!(query.insert_fields == vec![vec![String::from("1"), String::from("2"), String::from("3")],
    vec![String::from("4"), String::from("5"), String::from("6")]
    ]); 
}
#[test]

fn test_empty_failure(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "    ";
    let (_, error_code)  = p.parse(q);
    
    assert!(error_code == ast::ErrorCode::TokenizationError(String::from("Input query cannot be empty")));
    
}
#[test]

fn test_selec_without_from(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "SELECT ";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("Table name cannot be empty")));
    
}
#[test]
fn test_selec_without_fields(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "SELECT from a ";
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at SELECT: expected field to select")));
    
}

#[test]
fn test_selec_without_fields_comma(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "SELECT b, from a ";
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at SELECT: expected field to select")));
    
}
#[test]

fn test_selec_without_where_condition(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "SELECT b, from a WHERE";
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at SELECT: expected field to select")));
    
}
#[test]

fn test_selec_without_where_condition_op(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "SELECT b, from a WHERE c";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at SELECT: expected field to select")));
    
}
#[test]

fn update_empty(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("Table name cannot be empty")));
    
}
#[test]

fn update_empty_where(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE a";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at WHERE: empty where clause")));
    
}
#[test]
fn update_empty_where_set(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE a SET";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at WHERE: empty where clause")));
    
}
#[test]
fn update_empty_where_set_where(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE a SET b WHERE";
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at UPDATE: expected '='")));
    
}
#[test]

fn update_empty_where_set_where_a(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "UPDATE a SET b WHERE a";
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at UPDATE: expected '='")));
    
}
#[test]

fn delete_empty(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "DELETE FROM";
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("Table name cannot be empty")));
    
}
#[test]

fn delete_empty_without_where(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "DELETE FROM a";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at WHERE: empty where clause")));
    
}
#[test]

fn empty_insert_table(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("Table name cannot be empty")));
    
}
#[test]

fn empty_insert(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a'";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at INSERT INTO: need atleast one row to insert")));
    
}
#[test]

fn empty_insert_2(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' ( b";
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at INSERT INTO: need atleast one row to insert")));
    
}
#[test]

fn empty_insert_3(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' ( b) " ;
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at INSERT INTO: need atleast one row to insert")));
    
}
#[test]

fn empty_insert_4(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' ( b) VALUES" ;
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at INSERT INTO: need atleast one row to insert")));
    
}

#[test]

fn empty_insert_5(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' (b) VALUES (" ;
    let (_, error_code)  = p.parse(q);
    println!("{:?}", error_code);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("aat INSERT INTO: value count doesn't match field count")));
    
}

#[test]

fn empty_insert_6(){
    time_test!();
    let p = parser::SQLParser::new();
    let q = "INSERT INTO 'a' (*) VALUES (`1`)" ;
    let (_, error_code)  = p.parse(q);
    assert!(error_code == ast::ErrorCode::ParseError(String::from("at INSERT INTO: expected at least one field to insert")));
    
}









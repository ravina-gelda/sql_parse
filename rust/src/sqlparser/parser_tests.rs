
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
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
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
    // println!("{}",query.table);
    // println!("{:?}",query.fields);
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




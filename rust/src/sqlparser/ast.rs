pub enum QueryType{
    Unknown,
    SELECT, 
    UPDATE, 
    Insert,
    Delete
}
pub enum Operator {
    unknown,
    eq,
    ne,
    gt,
    lt,
    gte,
    lte,

}

pub struct Condition {
    operand1: String,
    operator:  Operator,
    operand2: String,
    operand_1_is_field : bool,
    operand_2_is_field : bool


}
pub struct Query{

    pub query_type : QueryType,
    pub table: String,
    pub conditions: Vec<Condition>,
    //TODO: add necessary fields for the UPDATE and delete queries. 
    pub fields: Vec<String> ,
    
}
impl Query {
    pub fn new()->Query {
        let mut q = Query{
            query_type: QueryType::Unknown,
            table: String::from(""),
            conditions: vec![],
            fields : vec![]

        };
        q

    }

}
#[derive(PartialEq, Eq)]
pub enum ErrorCode{
    None,
    TokenizationError,
    ParseError,
}





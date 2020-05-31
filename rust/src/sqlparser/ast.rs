pub enum QueryType{
    Unknown,
    SELECT, 
    UPDATE, 
    INSERT,
    DELETE
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
    pub operand1: String,
    pub operator:  Operator,
    pub operand2: String,
    pub operand_1_is_field : bool,
    pub operand_2_is_field : bool


}
impl Condition {
    pub fn new()-> Condition {
        let mut c  = Condition {
            operand1: String::new(),
            operator: Operator::unknown,
            operand2: String::new(),
            operand_1_is_field: false,
            operand_2_is_field: false,
         };
        return c;
    }


}
pub struct Query{

    pub query_type : QueryType,
    pub table: String,
    pub conditions: Vec<Condition>,
    //TODO: add necessary fields for the UPDATE and delete queries. 
    pub fields: Vec<String> ,
    pub update_fields : std::collections::HashMap<String,String>,
    pub insert_fields : Vec< Vec<String>>,
    
}
impl Query {
    pub fn new()->Query {
        let mut q = Query{
            query_type: QueryType::Unknown,
            table: String::from(""),
            conditions: vec![],
            fields : vec![], 
            update_fields : std::collections::HashMap::new(),
            insert_fields : vec![]

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





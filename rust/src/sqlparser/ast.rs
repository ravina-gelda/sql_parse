pub enum QueryType{
    Unknown,
    Select, 
    Update, 
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
    //TODO: add necessary fields for the update and delete queries. 
    pub fields: Vec<String> ,
}
#[derive(PartialEq, Eq)]
pub enum ErrorCode{
    None
}





#[macro_use]
extern crate time_test;
mod sqlparser;
use sqlparser::parser::* ;
fn main() {
    let q1 = "SELECT * from a";
    let q2 = "Select * from b";
    let queries = vec![q1,q2];
    let p = SQLParser::new();
    for k in queries.iter(){
       let (q,e) =  p.parse(k);
       println!("{}", q.table);

    }
}

#[macro_use]
extern crate time_test;
mod sqlparser;
use sqlparser::parser::* ;
fn main() {
    let q1 = "select a, c, d, e, f, g FROM 'b' where a < '1' and b < '2' and c < '3' and d < '4'";
    let q2 = "select a, c, d, e, f, g FROM 'b' where a < '1' and b < '2' and c < '3' and d < '4'";
    let q3 = "UPDATE 'a' SET b = 'hello', c = 'bye', d = 'then' WHERE a = '1' AND b = '789'";
    let q4 = "UPDATE 'a' SET b = 'hello', c = 'bye', d = 'then' WHERE a = '1' AND b = '789'";
    let q5 = "DELETE FROM 'a' WHERE b = '1' AND c < '3' and d < '4'";
    let q6 = "select a, c, d, e, f, g FROM 'b' where a < '1' and b < '2' and c < '3' and d < '4'";
    let q7 = "select a, c, d, e, f, g FROM 'b' where a < '1' and b < '2' and c < '3' and d < '4'";
    let q8 = "UPDATE 'a' SET b = 'hello', c = 'bye', d = 'then' WHERE a = '1' AND b = '789'";
    let q9 = "UPDATE 'a' SET b = 'hello', c = 'bye', d = 'then' WHERE a = '1' AND b = '789'";
    let q10 = "DELETE FROM 'a' WHERE b = '1' AND c < '3' and d < '4'";
    let queries = vec![q1,q2, q3, q4, q5, q6, q7, q8, q9, q10];
    let p = SQLParser::new();
    for k in queries.iter(){
       let (q,e) =  p.parse(k);
    }
}

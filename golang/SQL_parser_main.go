package main
import (
	"fmt"
	"log"

	"parser"
)

func main() {
    // PrintMemUsage()
   // var sql string ="SELECT a, c, d FROM 'b' WHERE a > '1'"
    
//    PrintMemUsage()
   //sqlparser.Parse("SELECT a, b, c FROM 'd' WHERE e = '1' AND f > '2'")
    query, err := parser.Parse("SELECT a, b, c FROM 'd' WHERE e = '1' AND f > '2'")
	
	//fmt.Printf("%+#v", query)
    
    //fmt.Println(err)
    if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%+#v", query)
  

  //  PrintMemUsage()
   
     
}

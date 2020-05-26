package main
import (
	"fmt"
	"log"
    "sql"
    //"query"
    "encoding/csv"
   "io"
   "os"
    //"reflect"
   //"math/rand"
   //"time"

	//"github.com/marianogappa/sqlparser"
)
func main() {
	query, err := sql.Parse("SELECT gdp FROM 'd' ")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%+#v", query)
    //new
    // Loading csv file
  rFile, err := os.Open("scatterdata.csv") //3 columns
  if err != nil {
    fmt.Println("Error:", err)
    return
   }
  defer rFile.Close()

  // Creating csv reader
  r := csv.NewReader(rFile)
    //r := csv.NewReader(strings.NewReader(in))
/*
	for {
		record, err := r.Read()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(record)
	}
    */
    lines, err := r.ReadAll()
  if err == io.EOF {
      fmt.Println("Error:", err)
      return
  }
    fmt.Println(query.Fields)
     fmt.Println(query.Type)
    //fmt.Println(lines.Columns())
    //fmt.Println(if lines[0][0]==query.Fields[0])
    var selected_columns [][]string
    if query.Type == 1 {
        //fmt.Println(query.Fields)
       for i := 0; i < len(query.Fields); i++ {
           //fmt.Println("Error:", i)
            for j:=0; j< len(lines[0]) ; j++{
             //   fmt.Println("Errorj:", j)
                if lines[0][j]==query.Fields[i]{
                    selected_columns=append(selected_columns,lines[0:2][1])
                }
            }
    }
        fmt.Printf("strArray1: %v\n", selected_columns)
    }
   
    
}
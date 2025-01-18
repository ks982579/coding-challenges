package main

import "fmt"

func main() {
  fmt.Println("FizzBuzz challenge in Go!");
  // first print all numbers between one and 100
  for i := 1; i <= 100; i++ {
    var message string = "";
    if i % 3 == 0 {
      message += "Fizz";
    }
    if i % 5 == 0 {
      message += "Buzz";
    }

    if message != "" {
      fmt.Println(message);
    } else {
      fmt.Println(i);
    }
  }
}

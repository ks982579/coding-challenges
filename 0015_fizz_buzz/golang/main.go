package main

import "fmt"

func main() {
  fmt.Println("FizzBuzz challenge in Go!");
  // first print all numbers between one and 100
  for i := 1; i <= 100; i++ {
    if i % 3 == 0 {
      fmt.Println("Fizz");
    } else {
      fmt.Println(i);
    }
  }
}

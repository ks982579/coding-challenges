using System;

class EntryPoint {
  static void Main() {
    // Printing 
    Console.WriteLine("Hello, World 2.0");
    // Array
    int[] info = { 5, 7, 10, 13, 15 };
    int target = 24;
    bool flag = false;

    // Doing double loop for now
    // Don't need to check last, it'll be checked.
    for ( int i = 0; i < info.Length - 1; i++ ) {
      if ( flag ) {
        break;
      }
      Console.WriteLine(info[i]);
      for ( int j = i + 1; j < info.Length; j++) {
        Console.WriteLine("  " + info[j]);
        if( info[i] + info[j] == target ) {
          flag = true;
          Console.WriteLine("Index Values: " + i + ", " + j);
          break;
        }
      }
    }
    if ( !flag ) {
      Console.WriteLine("No Solution");
    }
  }
}

using System;

class EntryPoint {
  static void Main() {
    // Printing 
    Console.WriteLine("Running Program...");
    // Array
    TwoSum( new int[] { 5, 7, 10, 13, 14 }, 24);
    TwoSum( new int[] { 5, 7, 10, 13, 12 }, 4);
    TwoSum( new int[] { 5, 7, 10, 13, 15 }, 13 + 15);
    Console.WriteLine("Program Complete");
  }

  static void TwoSum(int[] nums, int target) {
    bool flag = false;
    // Doing double loop for now
    // Don't need to check last, it'll be checked.
    for ( int i = 0; i < nums.Length - 1; i++ ) {
      if ( flag ) {
        break;
      }
      // Console.WriteLine(nums[i]);
      for ( int j = i + 1; j < nums.Length; j++) {
        // Console.WriteLine("  " + nums[j]);
        if( nums[i] + nums[j] == target ) {
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

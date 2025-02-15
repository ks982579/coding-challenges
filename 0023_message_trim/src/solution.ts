function solution(message: string, K: number): string {
  // If you cannot fit ' ...' - only print '...'
  const len: number = message.length;
  if (K < 4) {
    return "...";
  } else if (K >= len) {
    return message;
  }
  const tMessage = message.trim();
  // split by space - Assuming single spaces only.
  const mList: Array<string> = tMessage.split(" ");

  // to count words
  let charCount: number = 0;
  let sBuilder: string = "";

  for (let word of mList) {
    // Additional 1 for the ending space?
    let tempCount: number = charCount + word.length;
    if (tempCount > K - 5) {
      if (sBuilder.length > 0) {
        // Only add space if not first word
        sBuilder += " ";
      }
      return sBuilder + "...";
    } else {
      if (sBuilder.length > 0) {
        // Only add space if not first word
        sBuilder += " ";
      }
      sBuilder += word;
      charCount = sBuilder.length;
    }
  }
  return sBuilder;
}

const A: string = solution("And now here is my secret", 15);
console.log(A, A.length);

const B: string = solution("abcdefghi 123 567 90", 15);
console.log(B, B.length);

const C: string = solution("abcdefghi 123 567 90", 10);
console.log(C, C.length);

const D: string = solution("super dog", 4);
console.log(D, D.length);

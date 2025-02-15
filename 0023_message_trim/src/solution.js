"use strict";
function solution(message, K) {
    // If you cannot fit ' ...' - only print '...'
    const len = message.length;
    if (K < 4) {
        return "...";
    }
    else if (K >= len) {
        return message;
    }
    const tMessage = message.trim();
    // split by space - Assuming single spaces only.
    const mList = tMessage.split(" ");
    // to count words
    let charCount = 0;
    let sBuilder = "";
    for (let word of mList) {
        // Additional 1 for the ending space?
        let tempCount = charCount + word.length;
        if (tempCount > K - 5) {
            return sBuilder + " ...";
        }
        else {
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
const A = solution("And now here is my secret", 15);
console.log(A, A.length);
const B = solution("abcdefghi 123 567 90", 15);
console.log(B, B.length);
const C = solution("abcdefghi 123 567 90", 10);
console.log(C, C.length);
const D = solution("super dog", 4);
console.log(D, D.length);

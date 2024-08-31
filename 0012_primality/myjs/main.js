/* returns 0 if number is prime.
 * returns 1 if number is not prime
 * returns 2-3 for invalid input.
 */
let checker = (x) => {
  if (x < 2) {
    return 2;
  }
  if (!Number.isInteger(x)) {
    return 3;
  }
  for (let i = 2; i <= Math.sqrt(x); i++) {
    if (x % i === 0) {
      return 1;
    }
  }
  return 0;
};

function main() {
  let inputs = [-4, 0, 1, 2, 3, 3.5, 5, 6, 6.2, 15, 17, 18, 21, 23];
  for (let input of inputs) {
    let r = checker(input);
    if (r > 1) {
      console.log("Invalid input");
    } else {
      console.log(`${input} is ${r === 0 ? "prime" : "not prime"}`);
    }
  }
}

main();

import fs from "fs";

const input = fs
  .readFileSync("./src/2022/011-input.txt")
  .toString()
  .split("\n");

let current = 0;
let max = [0, 0, 0];

for (let i = 0; i < input.length; i++) {
  if (!input[i]) {
    max.sort();

    for (let k = 0; k < max.length; k++) {
      if (current > max[k]) {
        max[k] = current;
        console.log(max);
        break;
      }
    }

    current = 0;
  } else {
    current += parseInt(input[i]);
  }
}

console.log(max.reduce((a, i) => a + i, 0));

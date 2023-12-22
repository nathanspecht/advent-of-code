import fs from "fs";

const input = fs
  .readFileSync("./input.txt")
  .toString()
  .split("\n")
  .map((n) => parseInt(n));

let prev: number | null = null;
let count = 0;

input.forEach((value) => {
  if (prev === null) {
    prev = value;
    return;
  }

  if (value > prev) {
    count += 1;
  }

  prev = value;
});

console.log("Count:", count);

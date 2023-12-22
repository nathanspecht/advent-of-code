import fs from "fs";

const input = fs
  .readFileSync("src/day09/input.txt")
  .toString()
  .trim()
  .split("\n")
  .map((line) => line.split("").map((x) => parseInt(x)));

const width = input[0].length;
const height = input.length;

let sum = 0;

for (let y = 0; y < height; y++) {
  for (let x = 0; x < width; x++) {
    const point = input[y][x];

    if (isLow(y, x)) {
      sum += point + 1;
    }
  }
}

console.log("Answer:", sum);

function isLow(y: number, x: number) {
  let low = true;
  const point = input[y][x];

  console.log("Point:", point);

  for (const offset of [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
  ]) {
    const j = y + offset[0];
    const k = x + offset[1];

    if (j >= 0 && k >= 0 && j < height && k < width) {
      const compare = input[j][k];

      console.log(compare);

      if (compare <= point) {
        low = false;
      }
    }
  }

  console.log("Is low:", low);

  console.log("--");

  return low;
}

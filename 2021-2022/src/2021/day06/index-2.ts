import fs from "fs";

let counts: number[] = new Array(9).fill(0);

fs.readFileSync("inputs/input-06.txt")
  .toString()
  .trim()
  .split(",")
  .forEach((x) => (counts[parseInt(x)] += 1));

const DAYS = 256;

for (let day = 0; day < DAYS; day++) {
  const birthCount = counts.shift() ?? 0;
  counts[6] += birthCount;
  counts[8] = birthCount;
}

const total = counts.reduce((acc, x) => acc + x, 0);

console.log("Answer:", total);

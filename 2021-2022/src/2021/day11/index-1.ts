import fs from "fs";

const input = fs
  .readFileSync("src/day11/input.txt")
  .toString()
  .trim()
  .split("\n")
  .map((line) => line.split("").map((c) => parseInt(c)));

const STEPS = 100;

let flashes = 0;
let flashed: Record<string, [number, number]> = {};

for (let i = 0; i < STEPS; i++) {
  flashed = {};

  for (let y = 0; y < input.length; y++) {
    for (let x = 0; x < input[y].length; x++) {
      input[y][x] += 1;

      if (input[y][x] > 9) flash(y, x);
    }
  }

  for (const [y, x] of Object.values(flashed)) {
    input[y][x] = 0;
  }
}

function flash(y: number, x: number) {
  const key = `${y}:${x}`;

  if (flashed[key]) return;

  flashed[key] = [y, x];
  flashes += 1;

  for (
    let j = Math.max(0, y - 1);
    j <= Math.min(input.length - 1, y + 1);
    j++
  ) {
    for (
      let k = Math.max(0, x - 1);
      k <= Math.min(input[y].length - 1, x + 1);
      k++
    ) {
      const isSame = y === j && x === k;

      if (!isSame) {
        input[j][k] += 1;

        if (input[j][k] > 9) flash(j, k);
      }
    }
  }
}

console.log(flashes);

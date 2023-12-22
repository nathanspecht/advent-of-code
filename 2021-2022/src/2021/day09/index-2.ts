import fs from "fs";

const directions = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
];

const input = fs
  .readFileSync("src/day09/input.txt")
  .toString()
  .trim()
  .split("\n")
  .map((line) => line.split("").map((x) => parseInt(x)));

const width = input[0].length;
const height = input.length;

const lowPoints = [];

for (let y = 0; y < height; y++) {
  for (let x = 0; x < width; x++) {
    if (isLow(y, x)) {
      lowPoints.push([x, y]);
    }
  }
}

const basinSizes: number[] = [];

lowPoints.forEach(([x, y]) => {
  const checkedPoints: Record<string, boolean> = {};

  getBasinSize([x, y]);

  basinSizes.push(Object.keys(checkedPoints).length);

  function getBasinSize([x, y]: [number, number]) {
    const point = input[y][x];
    const key = `${x},${y}`;

    if (point === 9 || typeof checkedPoints[key] !== "undefined") return;

    checkedPoints[key] = true;

    for (const offset of directions) {
      const j = y + offset[0];
      const k = x + offset[1];

      if (j >= 0 && k >= 0 && j < height && k < width) {
        getBasinSize([k, j]);
      }
    }
  }
});

const answer = basinSizes
  .sort((a, b) => b - a)
  .slice(0, 3)
  .reduce((acc, x) => acc * x, 1);

console.log(answer);

function isLow(y: number, x: number) {
  let low = true;
  const point = input[y][x];

  for (const offset of directions) {
    const j = y + offset[0];
    const k = x + offset[1];

    if (j >= 0 && k >= 0 && j < height && k < width) {
      const compare = input[j][k];

      if (compare <= point) {
        low = false;
      }
    }
  }

  return low;
}

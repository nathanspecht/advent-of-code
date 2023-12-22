import fs from "fs";

type Point = [number, number];
type Line = [Point, Point];

const input = fs
  .readFileSync("./input-05.txt")
  .toString()
  .trim()
  .split("\n")
  .map((line) => {
    return line.split(" -> ").map((points) => {
      return points.split(",").map((x) => {
        const val = parseInt(x);
        if (isNaN(val)) throw new Error(`Invalid input: ${points}`);
        return val;
      });
    }) as Line;
  });

const output: Record<string, number> = {};

input.forEach((line) => {
  const points = drawLine(line);
  if (points) addLine(points);
});

let sum = 0;

for (const coords in output) {
  if (output[coords] > 1) {
    sum += 1;
  }
}

console.log("Answer:", sum);

function addLine(points: Record<string, number>) {
  for (const coords in points) {
    output[coords] ??= 0;
    output[coords] += points[coords];
  }
}

function drawLine(line: Line) {
  const [p1, p2] = line;
  const staticIndex = getStaticIndex(p1, p2);

  if (staticIndex === null) return null;

  const movingIndex = 1 - staticIndex;
  const range = [p1[movingIndex], p2[movingIndex]].sort((a, b) => {
    return a - b;
  });

  let points: Record<string, number> = {};

  for (let i = range[0]; i <= range[1]; i++) {
    const coords = [];
    coords[staticIndex] = p1[staticIndex];
    coords[movingIndex] = i;
    points[`${coords[0]},${coords[1]}`] = 1;
  }

  return points;
}

function getStaticIndex(p1: Point, p2: Point) {
  if (p1[0] === p2[0]) return 0;
  if (p1[1] === p2[1]) return 1;
  return null;
}

import fs from "fs";

type Point = [number, number];
type Line = [Point, Point];

const input = fs
  .readFileSync("inputs/input-05.txt")
  .toString()
  .trim()
  .split("\n")
  .map((line) => {
    return line.split(" -> ").map((points) => {
      return points.split(",").map((x) => parseInt(x));
    }) as Line;
  });

const output: Record<string, number> = {};

for (const line of input) {
  const points = drawLine(line);
  if (points) addLine(points);
}

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

  if (staticIndex !== null) {
    return drawHorizontalVertialLine(p1, p2, staticIndex);
  }

  if (isDiagonalLine(p1, p2)) {
    return drawDiagonalLine(p1, p2);
  }

  return null;
}

function drawHorizontalVertialLine(p1: Point, p2: Point, staticIndex: number) {
  const movingIndex = 1 - staticIndex;
  const range = sort([p1[movingIndex], p2[movingIndex]]);

  let points: Record<string, number> = {};

  for (let i = range[0]; i <= range[1]; i++) {
    const coords = [];
    coords[staticIndex] = p1[staticIndex];
    coords[movingIndex] = i;
    points[`${coords[0]},${coords[1]}`] = 1;
  }

  return points;
}

function drawDiagonalLine(_p1: Point, _p2: Point) {
  const [p1, p2] = _p1[0] < _p2[0] ? [_p1, _p2] : [_p2, _p1];

  const direction = p2[1] > p1[1] ? 1 : -1;

  let points: Record<string, number> = {};

  let j = p1[1];

  for (let i = p1[0]; i <= p2[0]; i++) {
    points[`${i},${j}`] = 1;
    j += direction;
  }

  return points;
}

function isDiagonalLine(p1: Point, p2: Point) {
  return Math.abs((p2[1] - p1[1]) / (p2[0] - p1[1]));
}

function getStaticIndex(p1: Point, p2: Point) {
  if (p1[0] === p2[0]) return 0;
  if (p1[1] === p2[1]) return 1;

  return null;
}

function sort(arr: number[]) {
  return [...arr].sort((a, b) => a - b);
}

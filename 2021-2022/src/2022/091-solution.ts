import fs from "fs";

type Position = { x: number; y: number };

const motions = fs
  .readFileSync("./src/2022/091-input.txt")
  .toString()
  .trim()
  .split("\n");

const up = { x: 0, y: -1 };
const down = { x: 0, y: 1 };
const left = { x: -1, y: 0 };
const right = { x: 1, y: 0 };

const directions: Record<string, Position> = {
  R: right,
  L: left,
  U: up,
  D: down,
};

const knots = [
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
  { x: 0, y: 0 },
];

const visitedPositions: Record<string, boolean> = { "0,0": true };

for (let i = 0; i < motions.length; i++) {
  const [direction, _steps] = motions[i].split(" ");
  const steps = parseInt(_steps);

  for (let j = 0; j < steps; j++) {
    knots[0] = applyDirection(knots[0], direction);

    for (let k = 1; k < knots.length; k++) {
      const currentKnotPosition = knots[k];
      const previousKnotPosition = knots[k - 1];

      const distanceX = distance(
        previousKnotPosition,
        currentKnotPosition,
        "x"
      );
      const distanceY = distance(
        previousKnotPosition,
        currentKnotPosition,
        "y"
      );

      if (Math.abs(distanceX) > 1 || Math.abs(distanceY) > 1) {
        const offset = { x: Math.sign(distanceX), y: Math.sign(distanceY) };
        knots[k] = applyOffset(currentKnotPosition, offset);

        if (k === knots.length - 1) {
          visitedPositions[`${knots[k].x},${knots[k].y}`] = true;
        }
      }
    }
  }
}

console.log(Object.keys(visitedPositions).length);

function distance(
  positionOne: Position,
  positionTwo: Position,
  axis: keyof Position
) {
  return positionOne[axis] - positionTwo[axis];
}

function applyDirection(position: Position, direction: string) {
  const offset = directions[direction];
  return applyOffset(position, offset);
}

function applyOffset(position: Position, offset: Position) {
  return { x: position.x + offset.x, y: position.y + offset.y };
}

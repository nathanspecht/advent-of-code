import fs from "fs";

let positions = fs
  .readFileSync("inputs/input-07.txt")
  .toString()
  .trim()
  .split(",")
  .map((x) => parseInt(x));

const max = Math.max(...positions);
const min = Math.min(...positions);

let minCost = Infinity;

for (let i = min; i <= max; i++) {
  const cost = positions.reduce((acc, pos) => {
    const distance = Math.abs(i - pos);
    const weightedDistance = getWeightedDistance(distance);

    return acc + weightedDistance;
  }, 0);

  if (cost < minCost) {
    minCost = cost;
  }
}

function getWeightedDistance(distance: number) {
  let weightedDistance = 0;

  for (let i = distance; i > 0; i--) {
    weightedDistance += i;
  }

  return weightedDistance;
}

console.log("Cost:", minCost);

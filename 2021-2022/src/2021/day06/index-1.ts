import fs from "fs";

let fish = fs
  .readFileSync("inputs/input-06.txt")
  .toString()
  .trim()
  .split(",")
  .map((x) => parseInt(x));

const DAYS = 80;

for (let day = 0; day < DAYS; day++) {
  let newFish = [];

  for (let i = 0; i < fish.length; i++) {
    if (fish[i] === 0) {
      fish[i] = 6;
      newFish.push(8);
    } else {
      fish[i] = fish[i] - 1;
    }
  }

  fish = [...fish, ...newFish];
}

console.log("Answer:", fish.length);

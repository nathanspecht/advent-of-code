import fs from "fs";

const input = fs
  .readFileSync("./input-01.txt")
  .toString()
  .trim()
  .split("\n")
  .map((n) => parseInt(n));

// const input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

let windows: number[][] = [];
let count = 0;

input.forEach((value) => {
  windows.push([]);

  windows.forEach((window) => {
    if (window.length < 3) {
      window.push(value);
    }
  });
});

console.log(windows);

windows.forEach((window, index) => {
  const prev = windows[index - 1];

  if (!prev) return;

  if (window.length === 3 && prev.length === 3) {
    if (sum(window) > sum(prev)) {
      count += 1;
    }
  }
});

function sum(arr: number[]) {
  return arr.reduce((a, b) => a + b, 0);
}

console.log("Count:", count);

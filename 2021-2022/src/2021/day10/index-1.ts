import fs from "fs";

const input = fs
  .readFileSync("src/day10/input.txt")
  .toString()
  .trim()
  .split("\n");

const scores: Record<string, number> = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
};

const pairs: Record<string, string> = {
  "(": ")",
  "[": "]",
  "{": "}",
  "<": ">",
};

const totalScore: number = input.reduce((acc, line) => {
  const stack = [];

  for (const char of line) {
    if (pairs[char]) {
      stack.unshift(char);
    } else if (stack[0] && pairs[stack[0]] === char) {
      stack.shift();
    } else {
      return acc + scores[char];
    }
  }

  return acc;
}, 0);

console.log(totalScore);

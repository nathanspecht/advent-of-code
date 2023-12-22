import fs from "fs";

const input = fs
  .readFileSync("src/day10/input.txt")
  .toString()
  .trim()
  .split("\n");

const scores: Record<string, number> = {
  ")": 1,
  "]": 2,
  "}": 3,
  ">": 4,
};

const pairs: Record<string, string> = {
  "(": ")",
  "[": "]",
  "{": "}",
  "<": ">",
};

const totalScores: number[] = input.flatMap((line) => {
  const stack = [];

  for (const char of line) {
    if (pairs[char]) {
      stack.unshift(char);
    } else if (stack[0] && pairs[stack[0]] === char) {
      stack.shift();
    } else {
      // ignore corrupted line
      return [];
    }
  }

  // remaining stack items need to be closed
  return stack.reduce((score, char) => {
    return score * 5 + scores[pairs[char]];
  }, 0);
});

totalScores.sort((a, b) => a - b);

console.log(totalScores[Math.floor(totalScores.length / 2)]);

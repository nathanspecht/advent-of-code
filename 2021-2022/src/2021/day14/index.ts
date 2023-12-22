import fs from "fs";

const [template, rulesDefs] = fs
  .readFileSync("src/day14/input.txt")
  .toString()
  .trim()
  .split("\n\n");

const rules: Record<string, string> = rulesDefs
  .split("\n")
  .reduce((defs, current) => {
    const [pair, result] = current.split(" -> ");
    return { ...defs, [pair]: result };
  }, {});

const STEPS = 10;

let polymer = template.split("");

const letterCounts = polymer.reduce<Record<string, number>>(countLetter, {});

for (let step = 0; step < STEPS; step++) {
  console.log("Step:", step);
  console.log("Count:", polymer.length);
  const insertions: string[] = [];

  for (let i = 0; i < polymer.length - 1; i++) {
    const pair = polymer.slice(i, i + 2).join("");
    const insertion = rules[pair];

    countLetter(letterCounts, insertion);

    insertions.push(insertion);
  }

  insertions.forEach((insertion, index) => {
    polymer.splice(index * 2 + 1, 0, insertion);
  });
}

let MIN = Infinity;
let MAX = 0;

for (const letter in letterCounts) {
  if (letterCounts[letter] <= MIN) MIN = letterCounts[letter];
  if (letterCounts[letter] >= MAX) MAX = letterCounts[letter];
}

console.log("MAX:", MAX);
console.log("MIN:", MIN);
console.log("MAX - MIN:", MAX - MIN);

function countLetter(counts: Record<string, number>, letter: string) {
  if (!counts[letter]) counts[letter] = 0;
  counts[letter] += 1;

  return counts;
}

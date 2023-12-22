import fs from "fs";

const input = fs
  .readFileSync("./src/2022/031-input.txt")
  .toString()
  .trim()
  .split("\n");

let sumOfPriorities = 0;

for (let i = 0; i < input.length - 2; i += 3) {
  const duplicateItem = findDuplicateItem(input[i], input[i + 1], input[i + 2]);
  sumOfPriorities += getPriority(duplicateItem);
}

console.log(sumOfPriorities);

function findDuplicateItem(one: string, two: string, three: string) {
  const firstMatches = one.match(new RegExp(`[${two}]`, "g"));

  if (!firstMatches) throw new Error("No matches");

  const secondMatches = three.match(new RegExp(`[${firstMatches.join("")}]`));

  if (!secondMatches) throw new Error("No second matches");

  return secondMatches[0];
}

function getPriority(item: string) {
  if (item >= "a" && item <= "z") {
    return item.charCodeAt(0) - "a".charCodeAt(0) + 1;
  }

  if (item >= "A" && item <= "Z") {
    return item.charCodeAt(0) - "A".charCodeAt(0) + 27;
  }

  throw new Error(`Invalid item: ${item}`);
}

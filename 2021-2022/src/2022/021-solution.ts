import fs from "fs";

const input = fs
  .readFileSync("./src/2022/021-input.txt")
  .toString()
  .trim()
  .split("\n");

const baseScore: Record<string, number> = {
  X: 1,
  Y: 2,
  Z: 3,
};

const scoreCard: Record<string, number> = {
  // tie
  AX: 3,
  BY: 3,
  CZ: 3,
  // lose
  AZ: 0,
  BX: 0,
  CY: 0,
  // win
  AY: 6,
  BZ: 6,
  CX: 6,
};

let total = 0;

for (let i = 0; i < input.length; i++) {
  const [theirs, result] = input[i].split(" ");
  const mine = calculateMine(theirs, result);
  total += calculateScore(theirs, mine);
}

console.log(total);

function calculateMine(theirs: string, result: string) {
  const direction = result === "X" ? -1 : result === "Y" ? 0 : 1;
  const code =
    ((theirs.charCodeAt(0) - "A".charCodeAt(0) + direction + 3) % 3) +
    "X".charCodeAt(0);
  /*
  console.log(
    { A: "rock", B: "paper", C: "scissors" }[theirs],
    { X: "rock", Y: "paper", Z: "scissors" }[String.fromCharCode(code)],
    { X: "lose", Y: "draw", Z: "win" }[result]
  );
  */
  return String.fromCharCode(code);
}

function calculateScore(theirs: string, mine: string) {
  const score = baseScore[mine] + scoreCard[theirs + mine];
  return score;
}

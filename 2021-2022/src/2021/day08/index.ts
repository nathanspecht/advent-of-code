import fs from "fs";

// 1 -> 2 segments
// 4 -> 4
// 7 -> 3
// 8 -> 7
//
// 0 -> 6
// 2 -> 5
// 3 -> 5
// 5 -> 5
// 6 -> 6
// 9 -> 6

const lengthToDigit: Record<string, string> = {
  "2": "1",
  "4": "4",
  "3": "7",
  "7": "8",
};

const numInCommon = {
  "0": {
    "1": 2,
    "4": 3,
    "7": 3,
    "8": 6,
  },
  "2": {
    "1": 1,
    "4": 2,
    "7": 2,
    "8": 5,
  },
  "3": {
    "1": 2,
    "4": 3,
    "7": 3,
    "8": 5,
  },
  "5": {
    "1": 1,
    "4": 3,
    "7": 2,
    "8": 5,
  },
  "6": {
    "1": 1,
    "4": 3,
    "7": 2,
    "8": 6,
  },
  "9": {
    "1": 2,
    "4": 4,
    "7": 3,
    "8": 6,
  },
};

const input = fs
  .readFileSync("src/day08/input.txt")
  .toString()
  .trim()
  .split("\n")
  .map((line) => line.split(" | ").map((seg) => seg.split(" ")));

let sum = 0;

for (const entry of input) {
  const [signals, output] = entry;
  const knownDigits: Record<string, string> = {};

  for (const signal of signals) {
    const digit = lengthToDigit[String(signal.length)];

    if (digit) {
      knownDigits[digit] = signal;
    }
  }

  for (const signal of signals) {
    if (!lengthToDigit[String(signal.length)]) {
      const digit = checkSignal(signal, knownDigits);

      knownDigits[String(digit)] = signal;
    }
  }

  const decoder = invert(knownDigits);

  const value = parseInt(
    output
      .map((signal) => {
        const digit = parseInt(decoder[signal.split("").sort().join("")]);

        return digit;
      })
      .join("")
  );

  sum += value;
}

console.log("Answer:", sum);

function invert(obj: Record<string, string>) {
  const retObj: Record<string, string> = {};

  for (const key in obj) {
    retObj[obj[key].split("").sort().join("")] = key;
  }

  return retObj;
}

function checkSignal(signal: string, knownDigits: Record<string, string>) {
  for (const proposedDigit in numInCommon) {
    const common: Record<string, number> =
      numInCommon[proposedDigit as keyof typeof numInCommon];
    const keys = Object.keys(common);

    const allInCommon = keys.every((key) => {
      const b = knownDigits[key];
      const a = signal;
      const inCommon = countCommon(a, b);

      return inCommon === common[key];
    });

    if (allInCommon) {
      return proposedDigit;
    }
  }
}

function countCommon(a: string, b: string) {
  let count = 0;

  for (const letter of a) {
    if (b.includes(letter)) count += 1;
  }

  return count;
}

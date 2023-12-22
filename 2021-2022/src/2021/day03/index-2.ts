import fs from "fs";

const input = fs.readFileSync("./input-03.txt").toString().trim().split("\n");

const oxygenGeneratorRating = getRating(1);
const c02ScrubberRating = getRating(0);

console.log("Answer:", oxygenGeneratorRating * c02ScrubberRating);

function getRating(preference: 0 | 1) {
  const ratingBinary = getBinary(input, 0, preference);
  const rating = parseInt(ratingBinary, 2);

  return rating;
}

function getBinary(codes: string[], index: number, preference: 0 | 1): string {
  const mostCommon = filterDigit(codes, index, preference);
  const filteredCodes = filterByMostCommon(codes, index, mostCommon);

  if (filteredCodes.length === 1) return filteredCodes[0];

  if (index >= input[0].length) throw new Error("No code found");

  return getBinary(filteredCodes, index + 1, preference);
}

function filterDigit(codes: string[], index: number, preference: 0 | 1) {
  const sum = codes.reduce((acc, code) => {
    return acc + parseInt(code[index]);
  }, 0);

  if (sum === codes.length / 2) return preference;

  const half = codes.length / 2;
  const compare = preference === 1 ? sum > half : sum < half;

  return compare ? 1 : 0;
}

function filterByMostCommon(codes: string[], index: number, mostCommon: 0 | 1) {
  return codes.filter((code) => {
    return parseInt(code[index]) === mostCommon;
  });
}

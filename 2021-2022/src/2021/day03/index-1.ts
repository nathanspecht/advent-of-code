import fs from "fs";

const input = fs.readFileSync("./input-03.txt").toString().trim().split("\n");

const colSums = new Array(input[0].length).fill(null).map((_, i) => {
  return input.reduce((sum, a) => {
    return sum + parseInt(a[i]);
  }, 0);
});

const numRows = input.length;

const gamma = colSums.map((sum) => (sum > numRows / 2 ? 1 : 0));
const eps = gamma.map((g) => 1 - g);

const gammaBinary = gamma.join("");
const epsBinary = eps.join("");

const answer = parseInt(gammaBinary, 2) * parseInt(epsBinary, 2);

console.log("Answer:", answer);

import fs from "fs";

const forest = fs
  .readFileSync("./src/2022/081-input.txt")
  .toString()
  .trim()
  .split("\n")
  .map((row) => row.split("").map((x) => parseInt(x)));

let maxScenicScore = 0;

const up = { x: 0, y: -1 }
const down = { x: 0, y: 1 }
const left = { x: -1, y: 0 }
const right = { x: 1, y: 0 }

for (let i = 0; i < forest.length; i++) {
  if (i === 0 || i === forest.length - 1) {
    continue;
  }

  for (let j = 0; j < forest[i].length; j++) {
    if (j === 0 || j === forest[i].length - 1) {
      continue;
    }

    const treeHeight = forest[i][j]
    const position = { x: j, y: i }

    const scenicScoreUp = getScenicScore(treeHeight, position, up, 0)
    const scenicScoreDown = getScenicScore(treeHeight, position, down, 0)
    const scenicScoreLeft = getScenicScore(treeHeight, position, left, 0)
    const scenicScoreRight = getScenicScore(treeHeight, position, right, 0)

    const scenicScore = scenicScoreUp * scenicScoreDown * scenicScoreLeft * scenicScoreRight

    if (scenicScore > maxScenicScore) maxScenicScore = scenicScore
  }
}

function getScenicScore(treeHeight: number, position: { x: number, y: number }, direction: { x: number, y: number }, count: number): number {
  const x = position.x + direction.x
  const y = position.y + direction.y

  if (y === -1 || y === forest.length) return count
  if (x === -1 || x === forest[y].length) return count
  if (forest[y][x] >= treeHeight) return count + 1

  return getScenicScore(treeHeight, { x, y }, direction, count + 1)
}

/*
function checkVisibility(treeHeight: number, position: { x: number, y: number }, direction: { x: number, y: number }): boolean {
  const x = position.x + direction.x
  const y = position.y + direction.y

  if (y === -1 || y === forest.length) return true
  if (x === -1 || x === forest[y].length) return true
  if (forest[y][x] >= treeHeight) return false

  return checkVisibility(treeHeight, { x, y }, direction)
}
*/

console.log(maxScenicScore)

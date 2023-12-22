import fs from "fs";

type Result = {
  lastDraw: number;
  board: (number | null)[][];
};

const input = fs.readFileSync("./input-04.txt").toString().trim().split("\n\n");

const inputs = input[0].split(",").map((x) => parseInt(x));

const boards = input
  .slice(1)
  .map((board) =>
    board.split("\n").map((row) => row.split(/ +/).map((x) => parseInt(x)))
  );

const result = play(0);

const sumOfUnmarked =
  result.board
    .flatMap((x) => x)
    .filter((x) => x !== null)
    .reduce((sum, a) => (sum === null || a === null ? a : sum + a), 0) ?? 0;

console.log("Answer:", sumOfUnmarked * result.lastDraw);

function play(index: number): Result {
  if (index >= inputs.length) throw new Error("No Winners");

  for (let i = 0; i < boards.length; i++) {
    const boardResult = checkBoard(boards[i], inputs.slice(0, index + 1));
    if (boardResult) return boardResult;
  }

  return play(index + 1);
}

function checkBoard(board: number[][], draws: number[]) {
  const result = board.map((row) =>
    row.map((tile) => (draws.includes(tile) ? null : tile))
  );
  const didWinRow = checkRows(result);
  const didWinCol = checkRows(invertBoard(result));

  if (didWinRow || didWinCol) {
    return { board: result, lastDraw: draws[draws.length - 1] };
  }

  return false;
}

function checkRows(board: (number | null)[][]) {
  return board.some((row) => row.filter((tile) => tile !== null).length === 0);
}

function invertBoard(board: (number | null)[][]) {
  const rowLength = board[0].length;

  return Array(rowLength)
    .fill(null)
    .map((_, i) => board.map((row) => row[i]));
}

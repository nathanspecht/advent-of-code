import fs from "fs";

type Assignment = [number, number];

const input = fs
  .readFileSync("./src/2022/041-input.txt")
  .toString()
  .trim()
  .split("\n");

let overlappingPairs = 0;

for (let i = 0; i < input.length; i++) {
  const [assignmentOne, assignmentTwo] = input[i]
    .split(",")
    .map(getAssignmentRange);

  if (
    doesLeftOverlap(assignmentOne, assignmentTwo) ||
    doesLeftOverlap(assignmentTwo, assignmentOne)
  ) {
    overlappingPairs += 1;
  }
}

console.log(overlappingPairs);

function doesLeftOverlap(
  assignmentLeft: Assignment,
  assignmentRight: Assignment
) {
  return (
    (assignmentLeft[0] <= assignmentRight[0] &&
      assignmentLeft[1] >= assignmentRight[1]) ||
    (assignmentLeft[0] <= assignmentRight[0] &&
      assignmentLeft[1] >= assignmentRight[0])
  );
}

function getAssignmentRange(assignment: string): Assignment {
  return assignment.split("-").map((x) => parseInt(x)) as Assignment;
}

import fs from "fs";

let [coordinates, foldingInstructions] = fs
  .readFileSync("src/day13/input.txt")
  .toString()
  .trim()
  .split("\n\n")
  .map((section) => section.split("\n"));

for (const instruction of foldingInstructions) {
  const [, axis, n] = instruction.match(/fold along (.)=([0-9]+)/)!;
  const pos = parseInt(n);

  coordinates = coordinates.flatMap((coord) => {
    let changed = false;
    let [x, y] = coord.split(",").map((n) => parseInt(n));

    if (axis === "x") {
      if (x > pos) {
        changed = true;
        x = 2 * pos - x;
      }
    } else {
      if (y > pos) {
        changed = true;
        y = 2 * pos - y;
      }
    }

    const key = `${x},${y}`;

    if (changed && coordinates.includes(key)) return [];

    return [`${x},${y}`];
  });
}

draw();

function draw() {
  const maxX = Math.max(...coordinates.map((x) => parseInt(x.split(",")[0])));
  const maxY = Math.max(...coordinates.map((x) => parseInt(x.split(",")[1])));

  let drawing = ``;

  for (let i = 0; i <= maxY; i++) {
    for (let j = 0; j <= maxX; j++) {
      if (coordinates.includes(`${j},${i}`)) {
        drawing += "#";
      } else {
        drawing += " ";
      }
    }

    drawing += "\n";
  }

  console.log(drawing);
}

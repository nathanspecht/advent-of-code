import fs from "fs";

type Command = { direction: "forward" | "down" | "up"; amount: number };
type Position = { horizontal: number; depth: number; aim: number };

const input = fs
  .readFileSync("./input-02.txt")
  .toString()
  .trim()
  .split("\n")
  .map((n): Command => {
    const [direction, amount] = n.split(" ");
    return {
      direction: direction as Command["direction"],
      amount: parseInt(amount),
    };
  });

const position: Position = {
  horizontal: 0,
  depth: 0,
  aim: 0,
};

input.forEach((command) => {
  assignCommand(command);
});

console.log(position);
console.log("Answer:", position.horizontal * position.depth);

function assignCommand(command: Command) {
  switch (command.direction) {
    case "up":
      position.aim -= command.amount;
      return;
    case "down":
      position.aim += command.amount;
      return;
    case "forward":
      position.horizontal += command.amount;
      position.depth += command.amount * position.aim;
      return;
  }
}

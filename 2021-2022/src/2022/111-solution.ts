import fs from "fs";
import "./031-solution.ts";
import "./071-solution.ts";

const NUM_ROUNDS = 20;

type Monkey = {
  name: string;
  items: number[];
  operator: string;
  operatorAmount: string;
  testDivisibleBy: number;
  trueMonkey: number;
  falseMonkey: number;
  numberInspected: number;
};

const inputs = fs
  .readFileSync("./src/2022/112-input.txt")
  .toString()
  .trim()
  .split("\n\n");

const monkeys: Monkey[] = inputs.map((input) => createMonkey(input));

console.log(monkeys)

for (let round = 0; round < NUM_ROUNDS; round++) {
  const monkeyIndex = round % monkeys.length;
  const monkey = monkeys[monkeyIndex];

  while (monkey.items.length > 0) {
    monkey.numberInspected += 1;
    let worryLevel = monkey.items.shift()!;
    if (typeof worryLevel === 'undefined') throw new Error(`no worry level: ${worryLevel}`)
    worryLevel = applyOperation(monkey, worryLevel);
    worryLevel = Math.floor(worryLevel / 3);
    const nextMonkeyIndex = testDivisible(monkey, worryLevel)
      ? monkey.trueMonkey
      : monkey.falseMonkey;
    giveItem(nextMonkeyIndex, worryLevel);
  }

  monkeys.forEach(monkey => {
    console.log(`${monkey.name}: ${monkey.items.join(', ')}`)
  })
}

console.log(monkeys.map((monkey) => monkey.numberInspected));

function testDivisible(monkey: Monkey, worryLevel: number) {
  return worryLevel % monkey.testDivisibleBy === 0
}

function applyOperation(monkey: Monkey, worryLevel: number): number {
  if (!isNaN(parseInt(monkey.operatorAmount))) {
    const operatorAmount = parseInt(monkey.operatorAmount)

    switch(monkey.operator) {
      case '+':
        console.log('Applying +', worryLevel, operatorAmount)
        const newWorryLevel = worryLevel + operatorAmount
        console.log('New worry level:', newWorryLevel)
        return newWorryLevel
      case '*':
        return worryLevel * operatorAmount
    }
  }

  if (monkey.operator === '*' && monkey.operatorAmount === 'old') {
    return worryLevel * worryLevel
  }

  throw new Error(`Unsupported operator: ${monkey.operator} ${monkey.operatorAmount}`)
}

function giveItem(monkeyIndex: number, worryLevel: number) {
  monkeys[monkeyIndex].items.push(worryLevel);
}

function createMonkey(input: string): Monkey {
  const lines = input.split("\n");
  const name = lines[0].match(/[\w\d ]+/g)![0];
  const items = lines[1].match(/\d+/g)!.map((item) => parseInt(item));
  const operators = lines[2].match(/[+\-*\/]|[\d+]/g)!;
  const operator = operators[0]
  const operatorAmount = operators[1] ?? 'old';
  const testDivisibleBy = parseInt(lines[3].match(/\d+/g)![0]);
  const trueMonkey = parseInt(lines[4].match(/\d+/g)![0]);
  const falseMonkey = parseInt(lines[5].match(/\d+/g)![0]);

  return {
    name,
    items,
    operator,
    operatorAmount,
    testDivisibleBy,
    trueMonkey,
    falseMonkey,
    numberInspected: 0,
  };
}

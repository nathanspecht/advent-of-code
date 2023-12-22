import fs from 'fs'

const input = fs
  .readFileSync("./src/2022/051-input.txt")
  .toString()
  .trim()
  .split("\n");

const stackLevels = input.slice(0, 8)

const stacks = [0, 1, 2, 3, 4, 5, 6, 7, 8].map(stackNumber => {
  return [0, 1, 2, 3, 4, 5, 6, 7].flatMap(level => {
    const crate =  stackLevels[level][stackNumber * 4 + 1]

    if (crate === ' ') return []

    return crate
  })
})

const instructions = input.slice(10)

for (let i = 0; i < instructions.length; i++) {
  const [count, fromStack, toStack] = instructions[i].match(/\d+/g)?.map(x => parseInt(x)) as number[]
  const fromIndex = fromStack - 1
  const toIndex = toStack - 1
  const cratesToMove = stacks[fromIndex].slice(0, count)//.reverse()

  stacks[fromIndex] = stacks[fromIndex].slice(count)
  stacks[toIndex] = [...cratesToMove, ...stacks[toIndex]]
}

console.log(stacks.map(stack => stack[0]).join(''))

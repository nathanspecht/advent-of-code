import fs from 'fs'

const input = fs
  .readFileSync("./src/2022/071-input.txt")
  .toString()
  .trim()
  .split('\n')

let directoryStack: string[] = ['/']
const sizes: Record<string, number> = {}
const fileCounted: Record<string, boolean> = {}

for (let i = 0; i < input.length; i++) {
  const args = input[i].split(' ')

  if (args[0] === '$') {
    if (args[1] === 'cd') {
      cd(args[2])
      continue
    }

    if (args[1] === 'ls') {
      continue
    }
  }

  if (args[0] === 'dir') {
    continue
  }

  if (!isNaN(parseInt(args[0]))) {
    const filePath = `${directoryStack.join('/')}/${args[1]}`
    addValueToStack(0, args[1], parseInt(args[0]), fileCounted[filePath])
    fileCounted[filePath] = true
    continue
  }

  throw new Error('Unrecognized input')
}

const spaceUsed = sizes['/']
const totalSpace = 70000000
const spaceNeeded = 30000000

const availableSpace = totalSpace - spaceUsed
const needToDelete = spaceNeeded - availableSpace

const smallestSizeToDelete = Object.values(sizes).reduce((acc, curr) => {
  if (curr >= needToDelete && curr < acc) {
    return curr
  }

  return acc
}, Infinity)

console.log(smallestSizeToDelete)

function addValueToStack(index: number, key: string, value: any, hasBeenCounted?: boolean) {
  if (index >= directoryStack.length) {
    throw new Error('Index out of directory stack range')
  }

  if (!isNaN(parseInt(value))) {
    const sizeKey = directoryStack.slice(0, index + 1).join('/')

    if (!sizes[sizeKey]) {
      sizes[sizeKey] = 0
    }

    sizes[sizeKey] += parseInt(value)
  }

  if (index === directoryStack.length - 1) {
    return
  }

  addValueToStack(index + 1, key, value, hasBeenCounted)
}

function cd(arg: string) {
  if (arg === '/') {
    directoryStack = ['/']
    return
  }

  if (arg === '..') {
    directoryStack.pop()
    return
  }

  directoryStack.push(arg)
}


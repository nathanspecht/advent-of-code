import fs from 'fs'

const input = fs
  .readFileSync("./src/2022/061-input.txt")
  .toString()
  .trim()

const uniqueLength = 14

const window: string[] = []

function findMarker() {
  for (let i = 0; i < input.length; i++) {
    window.push(input[i])

    if (window.length > uniqueLength) window.shift()

    if (window.length > uniqueLength) throw new Error("Too many elements in window")

    if (window.length === uniqueLength && isSignal(window)) {
      return i + 1
    }
  }

  console.log("No signal found")
}

function isSignal(window: string[]) {
  if (window.length !== uniqueLength) throw new Error(`Window must have length of ${uniqueLength}`)

  const record: Record<string, boolean> = {}

  for (let i = 0; i < window.length; i++) {
    if (record[window[i]]) return false
    record[window[i]] = true
  }

  return true
}

console.log(findMarker())

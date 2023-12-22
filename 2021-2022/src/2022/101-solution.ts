import fs from "fs";

type State = {
  x: number,
  signalIndex: number,
  currentSignal: string | null,
  timer: number,
  signalTotal: number
}

const NUM_CYCLES = 241
const CYCLES = [40, 80, 120, 160, 200, 240]

const signals = fs
  .readFileSync("./src/2022/101-input.txt")
  .toString()
  .trim()
  .split("\n");

const state: State = {
  x: 1,
  signalIndex: 0,
  currentSignal: null,
  timer: 0,
  signalTotal: 0
}

let currentRow: string[] = []

clearCurrentRow()

for (let cycle = 1; cycle <= NUM_CYCLES; cycle++) {

  const pixelPosition = (cycle - 1) % 40

  if (CYCLES.includes(cycle)) {
    console.log(currentRow.join(''))
    clearCurrentRow()
  }

  if (state.x - 1 <= pixelPosition && pixelPosition <= state.x + 1) {
    currentRow[pixelPosition] = '@'
  }

  if (state.currentSignal === null) {
    prepareSignal(signals[state.signalIndex])
    state.signalIndex += 1
  }

  tick()

  if (state.timer === 0) {
    executeSignal()
    state.currentSignal = null
  }
}

function clearCurrentRow() {
  currentRow = new Array(40).fill(' ')
}

function tick() {
  state.timer = Math.max(state.timer - 1, 0)
}

function prepareSignal(signal: string) {
  state.currentSignal = signal
  state.timer = signal === 'noop' ? 1 : 2
}

function executeSignal() {
  if (state.currentSignal?.startsWith('addx')) {
    const signal = state.currentSignal.split(' ')
    const value = parseInt(signal[1])

    state.x += value
  }
}

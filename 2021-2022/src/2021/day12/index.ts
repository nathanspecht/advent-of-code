import fs from "fs";

const input = fs
  .readFileSync("src/day12/input.txt")
  .toString()
  .trim()
  .split("\n");

const connections = setupConnections();
const numPaths = countPathsFrom(["start"]);

console.log(numPaths);

function countPathsFrom(points: string[]): number {
  const lastPoint = points[points.length - 1];

  if (lastPoint === "end") return 1;

  const availablePoints = connections[lastPoint].filter((connection) => {
    if (connection === "start") return false;

    const hasDuplicateLowerCase = hasDuplicates(
      points.filter((x) => !isUpperCase(x))
    );

    const canAddLowerCasePoint = hasDuplicateLowerCase
      ? !points.includes(connection)
      : count(connection, points) < 2;

    return isUpperCase(connection) || canAddLowerCasePoint;
  });

  if (availablePoints.length === 0) return 0;

  return availablePoints.reduce((acc, point) => {
    return acc + countPathsFrom([...points, point]);
  }, 0);
}

function setupConnections() {
  const _connections: Record<string, string[]> = {};

  for (const connection of input) {
    const [start, end] = connection.split("-");
    addConnection(start, end);
    addConnection(end, start);
  }

  function addConnection(a: string, b: string) {
    if (!_connections[a]) {
      _connections[a] = [b];
    } else {
      _connections[a].push(b);
    }
  }

  return _connections;
}

function count(item: string, list: string[]) {
  return list.filter((x) => x === item).length;
}

function isUpperCase(x: string) {
  return x === x.toUpperCase();
}

function hasDuplicates(list: string[]) {
  for (let i = 0; i < list.length; i++) {
    for (let j = 0; j < list.length; j++) {
      if (j !== i && list[i] === list[j]) {
        return true;
      }
    }
  }

  return false;
}

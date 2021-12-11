const { readFileSync } = require("fs");

const map = (readFileSync("input", "utf-8") as string)
    .split("\n")
    .filter(row => !!row)
    .map(row => row.split("").map(c => parseInt(c)));

const height = map.length;
const width = map[0].length;

const lowPoints: { x: number, y: number, h: number }[] = [];
for (let y = 0; y < height; ++y) {
    for (let x = 0; x < width; ++x) {
        const h = map[y][x];
        if ((x > 0 && map[y][x - 1] <= h) || (x + 1 < width && map[y][x + 1] <= h))
            continue;
        if ((y > 0 && map[y - 1][x] <= h) || (y + 1 < height && map[y + 1][x] <= h))
            continue;
        lowPoints.push({ x, y, h });
    }
}

console.log(lowPoints);

console.log(lowPoints.length, lowPoints.reduce((cc, p) => cc + p.h + 1, 0));

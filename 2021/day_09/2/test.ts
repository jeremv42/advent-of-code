const { readFileSync } = require("fs");

const map = (readFileSync("input", "utf-8") as string)
    .split("\n")
    .filter(row => !!row)
    .map(row => row.split("").map(c => parseInt(c)));

const height = map.length;
const width = map[0].length;

interface Point { x: number, y: number, h: number };

const lowPoints: Point[] = [];
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

const assigned: number[] = lowPoints.map(p => p.y * width + p.x);

function getNearHigherPoints({ x, y, h }: Point): Point[] {
    const points: Point[] = [];
    if (x > 0 && map[y][x - 1] !== 9 && !assigned.includes(y * width + x - 1))
        points.push({ y, x: x - 1, h: map[y][x - 1] });
    if (x + 1 < width && map[y][x + 1] !== 9 && !assigned.includes(y * width + x + 1))
        points.push({ y, x: x + 1, h: map[y][x + 1] });
    if (y > 0 && map[y - 1][x] !== 9 && !assigned.includes((y - 1) * width + x))
        points.push({ y: y - 1, x, h: map[y - 1][x] });
    if (y + 1 < height && map[y + 1][x] !== 9 && !assigned.includes((y + 1) * width + x))
        points.push({ y: y + 1, x, h: map[y + 1][x] });
    assigned.push(...points.map(p => p.y * width + p.x));
    return points;
}

const basins: number[] = [];
for (const point of lowPoints) {
    const points = [point];
    let toExplore = getNearHigherPoints(point);
    while (toExplore.length) {
        points.push(...toExplore);
        toExplore = [].concat(...toExplore.map(getNearHigherPoints));
    }
    basins.push(points.length);
}

console.log(basins.sort((a, b) => b - a), basins[0] * basins[1] * basins[2]);

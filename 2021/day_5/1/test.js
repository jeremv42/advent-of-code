const { readFileSync } = require("fs");

const lines = readFileSync("input", "utf-8").split("\n")
    .map(l => /(\d+),(\d+) -> (\d+),(\d+)/.exec(l))
    .filter(l => !!l)
    .map(l => l.map(s => s | 0))
    .map(([_, x1, y1, x2, y2]) => [{ x: x1, y: y1 }, { x: x2, y: y2 }]);

const xMax = lines.reduce((x, l) => Math.max(x, l[0].x, l[1].x), 0) + 1;
const yMax = lines.reduce((y, l) => Math.max(y, l[0].y, l[1].y), 0) + 1;

const map = new Array(yMax).fill(null).map(_ => new Array(xMax).fill(0));

function drawLine([{ x: x1, y: y1 }, { x: x2, y: y2 }]) {
    if (x1 !== x2 && y1 !== y2)
        return;
    if (x1 !== x2)
        for (let x = Math.min(x1, x2); x <= Math.max(x1, x2); ++x)
            map[y1][x] += 1;
    else
        for (let y = Math.min(y1, y2); y <= Math.max(y1, y2); ++y)
            map[y][x1] += 1;
}

lines.forEach(drawLine);

console.log(map);
console.log(map.reduce((c, row) =>
    c + row.reduce((cc, i) => i >= 2 ? cc + 1 : cc, 0)
    , 0
));

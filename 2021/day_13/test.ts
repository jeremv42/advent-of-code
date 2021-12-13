const fs = require("fs");

const lines = fs.readFileSync("input", "utf-8").split("\n");

const dots: { x: number, y: number }[] = [];
for (const line of lines) {
    if (line === "")
        break;
    const coords = line.split(",");
    dots.push({ x: parseInt(coords[0]), y: parseInt(coords[1]) });
}

const folds: ({ x: number } | { y: number })[] = [];
for (const line of lines.filter(l => l.startsWith("fold"))) {
    const [_, axis, nb] = /^fold along (x|y)=(\d+)$/.exec(line);
    folds.push({ [axis]: parseInt(nb) } as any);
}

let height = Math.max(...dots.map(d => d.y)) + 1;
let width = Math.max(...dots.map(d => d.x)) + 1;
let paper: boolean[][] = new Array(height).fill([]).map(_ => new Array(width).fill(false));
for (const {x, y} of dots) {
    paper[y][x] = true;
}

function foldY(y: number) {
    for (let i = 1; i <= y; ++i) {
        for (let x = 0; x < width; ++x) {
            paper[y - i][x] = paper[y - i][x] || paper[y + i][x];
        }
    }
    height = y;
    paper.length = y;
}
function foldX(x: number) {
    for (let i = 1; i <= x; ++i) {
        for (let y = 0; y < height; ++y) {
            paper[y][x - i] = paper[y][x - i] || paper[y][x + i];
        }
    }
    for (let y = 0; y < height; ++y)
        paper[y].length = x;
    width = x;
}
function fold(f: ({x: number} | {y: number})) {
    if ("x" in f)
        foldX(f.x);
    else
        foldY(f.y);
}
fold(folds[0]);
console.log(paper.map(r => r.map(d => d ? "#" : ".").join("")).join("\n"));
console.log(paper.reduce((cc, r) => cc + r.filter(d => d).length, 0));

folds.slice(1).map(fold);
console.log(paper.map(r => r.map(d => d ? "#" : ".").join("")).join("\n"));
console.log(paper.reduce((cc, r) => cc + r.filter(d => d).length, 0));

const fs = require("fs");

const arcs = fs.readFileSync("input", "utf-8").split("\n").map(l => l.split("-") as [string, string]);
const links = new Map<string, string[]>();
for (const [s, e] of arcs) {
    if (links.has(s))
        links.get(s).push(e);
    else
        links.set(s, [e]);

    if (links.has(e))
        links.get(e).push(s);
    else
        links.set(e, [s]);
}

const paths: string[] = [];

function findPaths(visited: string[], start: string) {
    visited = [...visited, start];
    for (const to of (links.get(start) ?? [])) {
        if (to === "end") {
            paths.push(`${visited.join("-")}-end`);
        }
        else if (!/^[a-z]/.test(to)) {
            findPaths(visited, to);
        }
        else if (!visited.includes(to))
            findPaths(visited, to);
    }
}

findPaths([], "start");
console.log(paths.join("\n"));
console.log(paths.length);

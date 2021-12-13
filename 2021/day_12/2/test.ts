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

const paths = new Set<string>();

function findPaths(path: string[], next: string, second: boolean) {
    if (next === "start")
        return;
    path = [...path, next];
    for (const to of (links.get(next) ?? [])) {
        if (to === "end") {
            paths.add(`${path.join("-")}-end`);
        }
        else if (!/^[a-z]/.test(to)) {
            findPaths(path, to, second);
        }
        else if (to !== "start") {
            if (!path.includes(to))
                findPaths(path, to, second);
            else if (!second)
                findPaths(path, to, true);
        }
    }
}


links.get("start").forEach(next => findPaths(["start"], next, false));
console.log(Array.from(paths).join("\n"));
console.log(paths.size);

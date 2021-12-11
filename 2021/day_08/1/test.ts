const { readFileSync } = require("fs");

const words = (readFileSync("input", "utf-8") as string)
    .split("\n")
    .filter(l => l.length)
    .map(line => line.split("|").map(str => str.split(" ")) as [string[], [string,string,string,string]]);

console.log(words.map(([s1, s2]) => `${s1?.join(" ")} || ${s2?.join(" ")}`));
console.log(words.reduce((cc, [_, s]) => cc + s.filter(w => [2, 3, 4, 7].includes(w.length)).length, 0));

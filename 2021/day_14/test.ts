const fs = require("fs");

const [template, ...lines] = fs.readFileSync("input", "utf-8").split("\n").filter(l => l);

const pairs = lines.map(l => l.split(" -> ") as [string, string]);
let polymers = new Map<string, number>(pairs.map(p => [p[0], 0]));

function addPair(polymers: Map<string, number>, pair: string, nb: number) {
    polymers.set(pair, polymers.get(pair) + nb);
}

for (let i = 1; i < template.length; ++i) {
    addPair(polymers, template[i - 1] + template[i], 1);
}

function step() {
    const newPolymers = new Map<string, number>(pairs.map(p => [p[0], 0]));
    for (const [pair, nb] of Array.from(polymers.entries())) {
        if (!nb)
            continue;
        const [, insert] = pairs.find(p => p[0] === pair);
        addPair(newPolymers, pair[0] + insert, nb);
        addPair(newPolymers, insert + pair[1], nb);
    }
    polymers = newPolymers;
}

console.log(`Step  0`, Array.from(polymers.entries()).map(([pair, nb]) => `${pair}:${nb.toString().padStart(3)}`).join("|"));

for (let i = 1; i <= 40; ++i) {
    step();

    console.log(`Step ${i.toString().padStart(2)}`, Array.from(polymers.entries()).map(([pair, nb]) => `${pair}:${nb.toString().padStart(3)}`).join("|"));
}

let elements: number[] = new Array<number>(26).fill(0);
const A = "A".charCodeAt(0);
for (const [pair, nb] of Array.from(polymers.entries())) {
    elements[pair.charCodeAt(0) - A] += nb;
}
elements[template.charCodeAt(template.length - 1) - A] += 1;

console.log(elements.map((n, i) => `${String.fromCharCode(A + i)}=${n}`).join(" "));
elements = elements.sort((a, b) => b-a).filter(e => e);
console.log(elements[0] - elements[elements.length-1]);

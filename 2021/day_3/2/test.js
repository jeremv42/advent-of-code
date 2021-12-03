const fs = require("fs");

const  lines = fs.readFileSync("input", "utf-8").split("\n");
const nbDigits = lines[0].length;

function getMostCommonPos(lines, i) {
    const avgOnes = lines
	  .map(l => l[i] === "1" ? 1 : 0)
	  .reduce((s, v) => s + v, 0) / lines.length;
    return avgOnes >= 0.5;
}

function getNb(func) {
    let nb = "";
    for (let i = 0; i < nbDigits; ++i) {
	const filteredLines = lines.filter(l => l.startsWith(nb));
	if (filteredLines.length === 1)
	    return filteredLines[0];
	const digit = func(filteredLines, i);
	console.log(`${i} => ${digit}`);
	nb += digit ? "1" : "0";
    }
    return nb;
}

function toNb(digits) {
    let nb = 0;
    for (let i = 0; i < digits.length; ++i)
	if (digits[i] === "1")
	    nb = nb | (1 << (digits.length - i - 1));
    return nb;
}

const o2 = toNb(getNb(getMostCommonPos));
const co2 = toNb(getNb((lines, i) => !getMostCommonPos(lines, i)));
console.log({ o2, co2, answer: o2*co2 });


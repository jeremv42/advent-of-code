const { readFileSync } = require("fs");

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/
const baseDigits = [
    "ABCEFG", // 0 - 6
    "CF",     // 1 - 2
    "ACDEG",  // 2 - 5
    "ACDFG",  // 3 - 5
    "BCDF",   // 4 - 4
    "ABDFG",  // 5 - 5
    "ABDEFG", // 6 - 6
    "ACF",    // 7 - 3
    "ABCDEFG",// 8 - 7
    "ABCDFG", // 9 - 6
];


const lines = (readFileSync("input", "utf-8") as string)
    .split("\n")
    .filter(l => l.length)
    .map(line => line.split(" | ").map(str => str.split(" ")) as [string[], string[]]);

function contains(digit: string, segments: string) {
    for (const s of segments)
        if (digit.indexOf(s) < 0)
            return false;
    return true;
}
function decode([digits, number]: [string[], string[]]): [string[], number] {
    const possibleDigits = baseDigits.map(_ => "");

    digits = digits.map(d => d.split("").sort().join(""));
    number = number.map(d => d.split("").sort().join(""));

    for (const digit of digits) {
        switch (digit.length) {
            case 2: possibleDigits[1] = digit; break;
            case 4: possibleDigits[4] = digit; break;
            case 3: possibleDigits[7] = digit; break;
            case 7: possibleDigits[8] = digit; break;
        }
    }

    possibleDigits[9] = digits.find(d => d.length === 6 && contains(d, possibleDigits[4]));
    possibleDigits[0] = digits.find(d => d.length === 6 && !possibleDigits.includes(d) && contains(d, possibleDigits[1]));
    possibleDigits[6] = digits.find(d => d.length === 6 && !possibleDigits.includes(d));
    possibleDigits[3] = digits.find(d => d.length === 5 && !possibleDigits.includes(d) && contains(d, possibleDigits[1]));
    possibleDigits[5] = digits.find(d => d.length === 5 && !possibleDigits.includes(d) && contains(possibleDigits[6], d));
    possibleDigits[2] = digits.find(d => d.length === 5 && !possibleDigits.includes(d));


    const nb = number.map(d => possibleDigits.indexOf(d));
    return [possibleDigits, nb[0] * 1000 + nb[1] * 100 + nb[2] * 10 + nb[3]];
}

let answer = 0;
for (const parts of lines) {
    const [digits, nb] = decode(parts);
    console.log(digits.join(" "), nb);
    answer += nb;
}
console.log(answer);

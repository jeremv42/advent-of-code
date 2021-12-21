const state = [
    {pos: 9, score: 0},
    {pos: 0, score: 0},
];

let dice = -1;
let rollCount = 0;

function roll() {
    rollCount += 1;
    dice = (dice + 1) % 100;
    return dice + 1;
}
function step(player: {pos: number, score: number}) {
    const d = roll() + roll() + roll();
    player.pos = (player.pos + d) % 10;
    player.score += player.pos + 1;
    return player.score >= 1000;
}

while (true) {
    if (step(state[0]) || step(state[1]))
        break;
}

console.log(rollCount, state);
console.log(rollCount * state[1].score);
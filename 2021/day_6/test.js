const { readFileSync } = require("fs");

const lanternfishes = readFileSync("input", "utf-8").split(",").map(i => parseInt(i));

const state = new Array(9).fill(0).map((_, i) => lanternfishes.filter(f => f === i).length);

function grow() {
	const tmp = state[0];
	for (let i = 1; i < state.length; ++i)
		state[i - 1] = state[i];
	state[6] += tmp;
	state[8] = tmp;
}

function printState() {
	console.log(state.map((nb, timer) => `${timer}=${nb}`).join(", "));
}

printState();
for (let d = 1; d <= 256; ++d) {
	grow(state);
	console.error(`day ${d} / total: ${state.reduce((cc, n) => cc + n, 0)}`);
	printState();
}

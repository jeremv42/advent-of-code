interface State {
    pos: number;
    score: number;
}

function hash(states: [State, State], player: number) {
    return states.map(s => s.pos.toString().padStart(2) + s.score.toString().padStart(2)).join(";") + player;
}

const cache = new Map<string, [number, number]>();
function calcUnivers(states: [State, State], player: number): [number, number] {
    const key = hash(states, player);
    if (cache.has(key))
        return cache.get(key);
    
    if (states[0].score >= 21)
        return [1, 0];
    if (states[1].score >= 21)
        return [0, 1];

    const wins: [number, number] = [0, 0];
    for (let d1 = 1; d1 <= 3; ++d1)
        for (let d2 = 1; d2 <= 3; ++d2)
            for (let d3 = 1; d3 <= 3; ++d3) {
                let copy = states.map(s => ({...s})) as [State, State];
                copy[player].pos = ((copy[player].pos + d1 + d2 + d3) % 10);
                copy[player].score += copy[player].pos + 1;

                const [w1, w2] = calcUnivers(copy, player ? 0 : 1);
                wins[0] += w1;
                wins[1] += w2;
            }

    cache.set(key, wins);
    return wins;
}


let states: [State, State] = [
    {pos: 10 - 1, score: 0},
    {pos: 1 - 1, score: 0}
];

console.log(calcUnivers(states, 0));

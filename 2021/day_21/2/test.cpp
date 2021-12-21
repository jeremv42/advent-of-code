#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <map>
#include <set>
#include <vector>
#include <unordered_map>

typedef std::array<uint64_t, 2> int2;

inline int64_t hash(int2 const& score, int2 const& pos, int player)
{
    return ((score[0]) + (pos[0] << 8) << 8) | ((score[1]) + (pos[1] << 8) << 24) | player;
}

std::unordered_map<int64_t, int2> cache;
int2 calcUnivers(int2 score, int2 pos, int player)
{
    auto key = hash(score, pos, player);
    auto it = cache.find(key);
    if (it != cache.end())
        return it->second;

    if (score[0] >= 21)
        return {1, 0};
    if (score[1] >= 21)
        return {0, 1};

    int2 wins = {0, 0};
    for (int d1 = 1; d1 <= 3; ++d1)
        for (int d2 = 1; d2 <= 3; ++d2)
            for (int d3 = 1; d3 <= 3; ++d3)
            {
                int2 copyPos = pos;
                int2 copyScore = score;
                copyPos[player] = ((pos[player] + d1 + d2 + d3) % 10);
                copyScore[player] += copyPos[player] + 1;

                auto w = calcUnivers(copyScore, copyPos, player ? 0 : 1);
                wins[0] += w[0];
                wins[1] += w[1];
            }

    cache[key] = wins;
    // std::cerr << std::hex << key << " / univers 1 = " << std::dec << wins[0] << " / univers 2 = " << wins[1] << std::endl;
    return wins;
}

int main()
{
    auto wins = calcUnivers({0, 0}, {10 - 1, 1 - 1}, false);
    std::cout << "univers 1 = " << wins[0] << " / univers 2 = " << wins[1] << std::endl;

    return 0;
}

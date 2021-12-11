#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <set>
#include <vector>

std::set<int> flashes;
std::vector<std::string> lines;
uint64_t score = 0;

int const neighboors[8][2] = {{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}};

bool in(int x, int y)
{
    return x >= 0 && y >= 0 && x < 10 && y < 10;
}

void flash(int x, int y);

void inc(int x, int y)
{
    if (flashes.find(x * 10 + y) != flashes.end())
        return;
    if (lines[y][x] == '9')
        flash(x, y);
    else
        lines[y][x] += 1;
}

void flash(int x, int y)
{
    if (flashes.find(x * 10 + y) != flashes.end())
        return;
    score += 1;
    flashes.emplace(x * 10 + y);
    lines[y][x] = '0';
    for (auto it : neighboors)
        if (in(x + it[0], y + it[1]))
            inc(x + it[0], y + it[1]);
}

void step()
{
    flashes.clear();
    for (int y = 0; y < 10; ++y)
        for (int x = 0; x < 10; ++x)
            inc(x, y);
}

int main()
{
    std::ifstream stream;
    stream.open("input");

    while (!stream.eof())
    {
        std::string line;
        std::getline(stream, line);
        lines.emplace_back(std::move(line));
    }

    for (int s = 0; s < 100; ++s)
    {
        std::cout << "\t\tstep " << s << "  /  " << score << std::endl;
        for (auto const& l : lines)
            std::cout << l << std::endl;
        step();
    }

    std::cout << "flashes = " << score << std::endl;
    return 0;
}

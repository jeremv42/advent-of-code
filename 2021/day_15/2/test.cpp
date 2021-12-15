#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <set>
#include <vector>

int height = 0;
int width = 0;
std::vector<std::vector<int64_t>> map;

int neighboors[4][2] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

bool in_bounds(int x, int y)
{
    return x >= 0 && y >= 0 && x < width && y < height;
}
bool in_bounds(std::tuple<int, int> const &tup)
{
    return in_bounds(std::get<0>(tup), std::get<1>(tup));
}

std::vector<std::vector<int64_t>> dijkstra()
{
    std::vector<std::vector<int64_t>> dist;
    dist.resize(height);
    for (auto &row : dist)
    {
        row.resize(width);
        std::fill(row.begin(), row.end(), -1);
    }

    std::vector<std::tuple<std::tuple<int, int>, std::tuple<int, int>, int64_t>> to_visit;

    dist[0][0] = 0;
    for (auto const &n : neighboors)
    {
        int x = n[0];
        int y = n[1];
        if (in_bounds(x, y))
            to_visit.emplace_back(std::make_tuple(0, 0), std::make_tuple(x, y), map[y][x]);
    }

    while (!to_visit.empty())
    {
        std::cout << to_visit.size() << std::endl;
        std::sort(std::begin(to_visit), std::end(to_visit), [&](auto const &a, auto const &b)
                      { return std::get<2>(b) < std::get<2>(a); });
        auto tup = to_visit.back();
        to_visit.pop_back();
        auto src = std::get<0>(tup);
        auto dst = std::get<1>(tup);
        auto score = std::get<2>(tup);
        dist[std::get<1>(dst)][std::get<0>(dst)] = score;
        if (std::get<0>(dst) == width - 1 && std::get<1>(dst) == height - 1)
            break;

        auto it = std::remove_if(to_visit.begin(), to_visit.end(), [&](auto& tup) {
            auto t = std::get<1>(tup);
            return std::get<0>(t) == std::get<0>(dst) && std::get<1>(t) == std::get<1>(dst);
        });
        to_visit.erase(it, to_visit.end());

        for (auto const &n : neighboors)
        {
            int x = n[0] + std::get<0>(dst);
            int y = n[1] + std::get<1>(dst);
            if (in_bounds(x, y) && dist[y][x] == -1)
            {
                to_visit.emplace_back(dst, std::make_tuple(x, y), score + map[y][x]);
            }
        }
    }

    return dist;
}

void printSolution(std::vector<std::vector<int64_t>> const &dist)
{
    for (int y = 0; y < height; ++y)
        for (int x = 0; x < width; ++x)
            std::cout << x << "x" << y << " \t\t" << dist[y][x] << std::endl;
    std::cout << "answer = " << dist[height - 1][width - 1] << std::endl;
}

int main()
{
    std::ifstream stream;
    stream.open("input");
    while (!stream.eof())
    {
        std::string line;
        std::getline(stream, line);
        if (line.size() < 5)
            continue;
        std::vector<int64_t> row(line.begin(), line.end());
        for (auto &c : row)
            c -= '0';

        auto w = row.size();
        row.resize(w * 5);
        for (auto i = 1; i < 5; ++i)
            for (auto j = 0; j < w; ++j)
                row[j + i*w] = std::max<int64_t>(1, (row[j + (i-1)*w] + 1) % 10);

        std::cerr << row.size() << std::endl;
        if (map.capacity() < w * 5)
            map.reserve(w * 5);
        map.emplace_back(std::move(row));
    }

    auto h = map.size();
    for (auto i = 0; i < 4; ++i)
    {
        for (auto j = 0; j < h; ++j)
        {
            auto row = map[j + h*i];
            for (auto& it: row)
                it = std::max<int64_t>(1, (it + 1) % 10);
            map.emplace_back(row);
        }
    }

    height = map.size();
    width = map[0].size();

    printSolution(dijkstra());
}

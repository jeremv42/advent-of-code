#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <set>
#include <vector>

typedef int64_t number;
int sum = 0;
std::vector<bool> bits;

int readInt(int& position, int size) {
    int nb = 0;
    while (size--)
        nb = (nb << 1) | bits[position++];
    return nb;
}

number readPacket(int& position)
{
    int version = readInt(position, 3);
    sum += version;
    int typeID = readInt(position, 3);
    // typeID, 4 = literal value, 6 (or other) = operator
    std::cout << "v" << version << " t" << typeID << " | ";
    if (typeID == 4) {
        bool more = true;
        number lit = 0;
        while (more)
        {
            more = bits[position++];
            number group = readInt(position, 4);
            lit = (lit * 16) + group;
        }
        std::cout << "literal=" << lit << std::endl;
        return lit;
    }
    else
    {
        int length = 0;
        std::vector<number> res;
        if (!bits[position++]) {
            length = readInt(position, 15);
            std::cout << "length=" << length;
            std::cout << std::endl;
            int end = position + length;
            while (position < end)
                res.push_back(readPacket(position));
        }
        else {
            length = readInt(position, 11);
            std::cout << "nb sub=" << length;
            std::cout << std::endl;
            while (length--) {
                res.push_back(readPacket(position));
            }
        }
        switch (typeID) {
            case 0: return std::accumulate(std::begin(res), std::end(res), number(0), [](number acc, number i) { return acc + i; });
            case 1: return std::accumulate(std::begin(res), std::end(res), number(1), [](number acc, number i) { return acc * i; });
            case 2: return std::accumulate(std::begin(res), std::end(res), number(999999999999999999999), [](number acc, number i) { return std::min(acc, i); });
            case 3: return std::accumulate(std::begin(res), std::end(res), number(0), [](number acc, number i) { return std::max(acc, i); });
            case 5: return res[0] > res[1];
            case 6: return res[0] < res[1];
            case 7: return res[0] == res[1];
        }
    }
    return 0;
}

int main()
{
    std::ifstream stream;
    stream.open("input");
    std::string line;
    std::getline(stream, line);
    bits.reserve(line.size() * 4);
    for (auto c : line)
    {
        int nb = std::isdigit(c) ? c - '0' : (c - 'A' + 10);
        bits.push_back(nb & 8);
        bits.push_back(nb & 4);
        bits.push_back(nb & 2);
        bits.push_back(nb & 1);
    }

    for (auto b : bits)
        std::cout << (int)b;
    std::cout << std::endl;

    int position = 0;
    auto res = readPacket(position);

    std::cout << sum  << " / " << res << std::endl;
    return 0;
}

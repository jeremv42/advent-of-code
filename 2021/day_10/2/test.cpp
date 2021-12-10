#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <vector>

struct Counters
{
    std::string expect = "";
};

int parse(std::string &expectStr, std::string const &str, int idx);

int parse_end(std::string &expectStr, std::string const &str, int idx, char expect)
{
    if (idx >= str.size())
        throw std::make_tuple('i', str[idx], idx);
    if (str[idx] != expect)
    {
        idx = parse(expectStr, str, idx);
        if (idx >= str.size())
            throw std::make_tuple('i', str[idx], idx);
        if (str[idx] != expect)
            throw std::make_tuple(expect, str[idx], idx);
    }
    expectStr = expectStr.substr(0, expectStr.size() - 1);
    return idx + 1;
}

int parse(std::string &expectStr, std::string const &str, int idx)
{
    if (idx >= str.size())
        return idx;
    switch (str[idx])
    {
    case '(':
        expectStr += ')';
        idx = parse(expectStr, str, idx + 1);
        return parse_end(expectStr, str, idx, ')');
    case '[':
        expectStr += ']';
        idx = parse(expectStr, str, idx + 1);
        return parse_end(expectStr, str, idx, ']');
    case '{':
        expectStr += '}';
        idx = parse(expectStr, str, idx + 1);
        return parse_end(expectStr, str, idx, '}');
    case '<':
        expectStr += '>';
        idx = parse(expectStr, str, idx + 1);
        return parse_end(expectStr, str, idx, '>');
    }
    return idx;
}

int main()
{
    std::ifstream stream;
    stream.open("input");
    std::vector<uint64_t> scores;

    while (!stream.eof())
    {
        std::string line;
        getline(stream, line);
        std::string expect;
        try
        {
            int idx = parse(expect, line, 0);
            while (idx < line.size())
                idx = parse(expect, line, idx);
        }
        catch (std::tuple<char, char, int> &err)
        {
            if (std::get<0>(err) != 'i')
                std::cerr << "Expected " << std::get<0>(err) << ", but found " << std::get<1>(err) << " instead (idx=" << std::get<2>(err) << ").\n";
            else
            {
                std::cerr << line << std::endl;
                uint64_t sc = std::accumulate(
                    expect.rbegin(), expect.rend(), (uint64_t)0,
                    [](auto cc, auto c) {
                        cc *= 5;
                        switch (c)
                        {
                            case ')': return cc + 1;
                            case ']': return cc + 2;
                            case '}': return cc + 3;
                            case '>': return cc + 4;
                        }
                        return cc;
                    });
                scores.push_back(sc);
                std::cout << "score = " << sc << " " << expect << std::endl;
                std::cerr << "Incomplete char=" << std::get<1>(err) << " (idx=" << std::get<2>(err) << ").\n";
            }
        }
    }

    std::sort(scores.begin(), scores.end());
    for (auto sc: scores)
        std::cout << sc << std::endl;
    std::cout << "score = " << scores[scores.size()/2] << std::endl;
    return 0;
}

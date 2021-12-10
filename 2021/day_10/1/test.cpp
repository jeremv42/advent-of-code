#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <vector>

struct Counters
{
    int parenthesis = 0;
    int bracket = 0;
    int curly = 0;
    int lessthan = 0;
};

int parse(Counters &counters, std::string const &str, int idx);

int parse_end(Counters &counters, std::string const &str, int idx, char expect)
{
    if (idx >= str.size())
        throw std::make_tuple('i', str[idx], idx);
    if (str[idx] != expect)
    {
        idx = parse(counters, str, idx);
        if (idx >= str.size())
            throw std::make_tuple('i', str[idx], idx);
        if (str[idx] != expect)
            throw std::make_tuple(expect, str[idx], idx);
    }

    switch (expect)
    {
    case ')':
        counters.parenthesis += 1;
    case ']':
        counters.bracket += 1;
    case '}':
        counters.curly += 1;
    case '>':
        counters.lessthan += 1;
    }
    return idx + 1;
}

int parse(Counters &counters, std::string const &str, int idx)
{
    if (idx >= str.size())
        return idx;
    switch (str[idx])
    {
    case '(':
        idx = parse(counters, str, idx + 1);
        return parse_end(counters, str, idx, ')');
    case '[':
        idx = parse(counters, str, idx + 1);
        return parse_end(counters, str, idx, ']');
    case '{':
        idx = parse(counters, str, idx + 1);
        return parse_end(counters, str, idx, '}');
    case '<':
        idx = parse(counters, str, idx + 1);
        return parse_end(counters, str, idx, '>');
    }
    return idx;
}

int main()
{
    std::ifstream stream;
    stream.open("input");
    int score = 0;

    while (!stream.eof())
    {
        std::string line;
        getline(stream, line);
        Counters counts;
        try
        {
            std::cerr << line << std::endl;
            if (parse(counts, line, 0))
            {
                std::cout << "parenthesis=" << counts.parenthesis << " ";
                std::cout << "bracket=" << counts.bracket << " ";
                std::cout << "curly=" << counts.curly << " ";
                std::cout << "lessthan=" << counts.lessthan << std::endl;
            }
        }
        catch (std::tuple<char, char, int> &err)
        {
            if (std::get<0>(err) != 'i')
            {
                switch (std::get<1>(err))
                {
                case ')':
                    score += 3;
                    break;
                case ']':
                    score += 57;
                    break;
                case '}':
                    score += 1197;
                    break;
                case '>':
                    score += 25137;
                    break;
                }
                std::cerr << "Expected " << std::get<0>(err) << ", but found " << std::get<1>(err) << " instead (idx=" << std::get<2>(err) << ").\n";
            }
            else
                std::cerr << "Incomplete char=" << std::get<1>(err) << " (idx=" << std::get<2>(err) << ").\n";
        }
    }

    std::cout << "score = " << score << std::endl;
    return 0;
}

/*
[({(<(())[]>[[{[]{<()<>>
{([(<{}[<>[]}>{[]{[(<()>
<{([([[(<>()){}]>(<<{{
*/
#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <map>
#include <set>
#include <vector>

std::string enhancement;
std::vector<std::string> input;

int enhanceCount = 1;
std::vector<std::string> createOutput(std::vector<std::string> const& input)
{
    auto width = input[0].size();
    auto height = input.size();
    std::vector<std::string> output;

    output.reserve(height + 4);
    output.emplace_back(std::string(width + 4, (enhanceCount++ % 2) ? enhancement[0] : enhancement[511]));
    for (auto i = 1; i < height + 4; ++i)
        output.emplace_back(output[0]);

    for (auto y = 1; y < height - 1; ++y)
    {
        for (auto x = 1; x < width - 1; ++x)
        {
            int idx = 0
                | ((input[y - 1][x - 1] == '#') << 8)
                | ((input[y - 1][x + 0] == '#') << 7)
                | ((input[y - 1][x + 1] == '#') << 6)
                | ((input[y + 0][x - 1] == '#') << 5)
                | ((input[y + 0][x + 0] == '#') << 4)
                | ((input[y + 0][x + 1] == '#') << 3)
                | ((input[y + 1][x - 1] == '#') << 2)
                | ((input[y + 1][x + 0] == '#') << 1)
                | ((input[y + 1][x + 1] == '#') << 0)
            ;
            output[y + 2][x + 2] = enhancement[idx];
        }
    }
    return output;
}

std::vector<std::string> enhance(std::vector<std::string> const& input)
{
    auto output = createOutput(input);
    for (auto l: output)
        std::cout << l << std::endl;
    std::cout << std::endl;
    return output;
}

int main()
{
    std::ifstream stream;
    stream.open("input");
    std::string line;
    std::getline(stream, line);
    do
    {
        enhancement += line;
        std::getline(stream, line);
    } while (line.size() > 0);
    input.emplace_back("");
    input.emplace_back("");
    input.emplace_back("");
    input.emplace_back("");
    while (!stream.eof())
    {
        std::getline(stream, line);
        input.emplace_back("...." + line + "....");
    }
    input[0] = std::string(input[4].size(), '.');
    input[1] = input[0];
    input[2] = input[0];
    input[3] = input[0];
    input[input.size() - 1] = input[0];
    input.push_back(input[0]);
    input.push_back(input[0]);
    input.push_back(input[0]);

    for (auto l: input)
        std::cout << l << std::endl;
    std::cout << std::endl;
    
    for (auto i = 0; i < 50; ++i)
        input = enhance(input);

    int count = 0;
    for (auto const& l: input)
        for (auto c: l)
            count += c == '#';
    std::cout << count << std::endl;
    return 0;
}
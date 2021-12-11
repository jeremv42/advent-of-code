#include <algorithm>
#include <numeric>
#include <vector>
#include <iostream>
#include <fstream>

using namespace std;

int sum(int pos, int to)
{
    pos = abs(pos - to);
    int fuel = 0;
    while (pos)
    {
        fuel += pos;
        pos -= 1;
    }
    return fuel;
}

int main()
{
    vector<int> positions;

    std::ifstream file("input");
    string str;
    while (getline(file, str, ','))
        positions.push_back(stoi(str));

    for (auto it: positions)
        cout << it << ",";
    cout << endl;

    int fuelMin = numeric_limits<int>::max();
    int pos = 0;
    int posMax = *max_element(begin(positions), end(positions));
    for (int i = 0; i < posMax; ++i)
    {
        int fuel = accumulate(begin(positions), end(positions), 0, [i](auto cc, auto p) { return cc + sum(p, i); });
        if (fuel < fuelMin)
        {
            pos = i;
            fuelMin = fuel;
        }
    }
    printf("pos = %d, fuel = %d\n", pos, fuelMin);

    return 0;
}
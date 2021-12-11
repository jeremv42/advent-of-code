#include <algorithm>
#include <numeric>
#include <vector>
#include <iostream>
#include <fstream>

using namespace std;

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

    sort(positions.begin(), positions.end());
    int mediane = (positions.size() % 2) ? (positions[positions.size()/2 - 1] + positions[positions.size()/2]) / 2 : positions[positions.size()/2];
    int fuel = accumulate(positions.begin(), positions.end(), 0, [mediane](auto cc, auto p) { return cc + abs(p - mediane); });
    printf("pos = %d, fuel = %d\n", mediane, fuel);

    return 0;
}
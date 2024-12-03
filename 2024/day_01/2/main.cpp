#include <iostream>
#include <map>
#include <vector>

using namespace std;

int main(int ac, char const *const *av)
{
	vector<int> l1;
	map<int, int> l2;

	while (!cin.eof())
	{
		int a;
		int b;
		cin >> a >> b;
		if (cin.good())
		{
			l1.push_back(a);
			l2[b] += 1;
		}
	}

	uint64_t sum = 0;
	for (auto a : l1)
	{
		sum += a * l2[a];
	}

	cout << sum << endl;

	return 0;
}
#include <iostream>
#include <vector>

using namespace std;

int main(int ac, char const *const *av)
{
	vector<int> l1;
	vector<int> l2;

	while (!cin.eof())
	{
		int a;
		int b;
		cin >> a >> b;
		if (cin.good())
		{
			l1.push_back(a);
			l2.push_back(b);
		}
	}

	sort(l1.begin(), l1.end(), [](auto a, auto b)
		 { return a < b; });
	sort(l2.begin(), l2.end(), [](auto a, auto b)
		 { return a < b; });

	uint64_t sum = 0;
	for (size_t i = 0; i < l1.size(); ++i)
	{
		sum += abs(l1[i] - l2[i]);
	}

	cout << sum << endl;

	return 0;
}
#include <iostream>
#include <map>
#include <vector>

using namespace std;

bool is_valid_inc(vector<int> const &report)
{
	for (size_t i = 1; i < report.size(); ++i)
	{
		auto diff = report[i] - report[i - 1];
		if (diff < 1 || diff > 3)
			return false;
	}
	return true;
}
bool is_valid_dec(vector<int> const &report)
{
	for (size_t i = 1; i < report.size(); ++i)
	{
		auto diff = report[i - 1] - report[i];
		if (diff < 1 || diff > 3)
			return false;
	}
	return true;
}

bool is_valid(vector<int> const &report)
{
	if (is_valid_dec(report) || is_valid_inc(report))
		return true;
	for (size_t i = 0; i < report.size(); ++i)
	{
		auto copy = report;
		copy.erase(copy.begin() + i);

		if (is_valid_dec(copy) || is_valid_inc(copy))
			return true;
	}
	return false;
}

int main(int ac, char const *const *av)
{
	vector<vector<int>> reports;
	while (cin.good())
	{
		int nb;
		cin >> nb;
		if (cin.good())
		{
			vector<int> report;
			report.push_back(nb);
			while (cin.get() != '\n' && cin.good())
			{
				cin >> nb;
				report.push_back(nb);
			}
			reports.push_back(report);
		}
	}

	int count = 0;
	for (auto &report : reports)
	{
		if (is_valid(report))
			count += 1;
	}
	cout << count << endl;

	return 0;
}

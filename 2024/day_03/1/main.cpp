#include <iostream>
#include <regex>

using namespace std;

int main(int ac, char const *const *av)
{
	string content((istreambuf_iterator<char>(cin)), istreambuf_iterator<char>());

	size_t sum = 0;

	regex mul_reg("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)");
	auto mul_it = sregex_iterator(content.begin(), content.end(), mul_reg);
	for (; mul_it != sregex_iterator(); ++mul_it)
	{
		auto match = *mul_it;
		sum += stoi(match[1].str()) * stoi(match[2].str());
	}

	cout << sum << endl;
	return 0;
}
#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <unordered_set>
#include <vector>


typedef std::array<int, 5*5> Board;
std::vector<int> numbers;
std::vector<Board> boards;

void printdBoard(Board const& board)
{
    for (int r = 0; r < 5; ++r)
    {
        for (int c = 0; c < 5; ++c)
        {
            std::cout << std::setfill(' ') << std::setw(3) << board[r*5 + c];
        }
        std::cout << std::endl;
    }
}

void playNumber(Board& board, int nb)
{
    for (auto& it: board)
        if (it == nb)
            it = -1;
}
bool isWinningBoard(Board& board)
{
    for (int r = 0; r < 5; ++r)
    {
        int c = 0;
        for (; c < 5; ++c)
            if (board[r*5 + c] != -1)
                break;
        if (c == 5)
            return true;
    }

    for (int c = 0; c < 5; ++c)
    {
        int r = 0;
        for (; r < 5; ++r)
            if (board[r*5 + c] != -1)
                break;
        if (r == 5)
            return true;
    }

    return false;
}

int main()
{
    std::ifstream stream;
    stream.open("input");
    std::string firstLine;
    getline(stream, firstLine);
    std::stringstream ss(firstLine);
    while (!ss.eof())
    {
        int nb;
        char c;
        ss >> nb >> c;
        numbers.push_back(nb);
        if (c != ',')
            break;
    }

    while (!stream.eof())
    {
        Board board;
        for (auto& it: board)
            stream >> it;
        boards.push_back(board);
    }

    for (auto& it: boards)
    {
        printdBoard(it);
        std::cout << std::endl;
    }

    std::unordered_set<Board*> winningBoards(numbers.size());
    Board last;
    int lastNb;
    for (auto nb: numbers)
    {
        for (auto& board: boards)
        {
            playNumber(board, nb);
            if (isWinningBoard(board) && winningBoards.find(&board) == winningBoards.end())
            {
                winningBoards.insert(&board);
                last = board;
                lastNb = nb;
            }
        }
    }

    auto& board = last;
    printdBoard(board);
    int sum = 0;
    for (auto n: board)
        if (n >= 0)
            sum += n;
    std::cout << "answer = " << (sum * lastNb) << std::endl;
    return 0;
}
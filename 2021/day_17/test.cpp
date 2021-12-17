#include <algorithm>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <set>
#include <vector>

typedef int number;

// target area: x=192..251, y=-89..-59
number target_min_x = 192;
number target_max_x = 251;
number target_min_y = -89;
number target_max_y = -59;

number vx = 0;
number vy = 0;
number ax = 0;
number step = 0;
number x = 0;
number y = 0;

bool in_bounds()
{
    return target_min_x <= x && x <= target_max_x && target_min_y <= y && y <= target_max_y;
}

void reset(number velocity_x, number velocity_y)
{
    vx = velocity_x;
    vy = velocity_y;
    ax = velocity_x > 0 ? -1 : (velocity_x < 0 ? 1 : 0);
    step = 0;
    x = 0;
    y = 0;
}

int main()
{
    number max_y = 0;
    number probes = 0;
    for (number xx = 1; xx <= target_max_x; ++xx)
    {
        for (number yy = target_max_y*2; yy <= target_max_y*-2; ++yy)
        {
            number tmp_y = 0;
            reset(xx, yy);
            while (x < target_max_x && (vx > 0 || x >= target_min_x) && y >= target_min_y)
            {
                x += vx;
                vx += ax;
                ax = vx == 0 ? 0 : ax;
                y += vy;
                vy -= 1;
                step += 1;
                tmp_y = std::max(tmp_y, y);

                if (in_bounds())
                    break;
            }

            if (in_bounds())
            {
                std::cout << step << " | " << xx << "x" << yy << " => " << tmp_y << std::endl;
                max_y = std::max(max_y, tmp_y);
                probes += 1;
            }
        }
    }

    std::cout << max_y << " / " << probes << std::endl;
}

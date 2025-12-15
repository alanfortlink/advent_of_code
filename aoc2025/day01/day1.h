#include "../utils.h"

#include <vector>

#pragma once

namespace aoc2025 {

using std::string;
using std::vector;

class Day1 {
public:
  int part1(const vector<string> &lines) {
    int current = 50;
    int zeroCount = 0;
    for (const string &line : lines) {
      char dir = line[0];
      int value = std::stoi(line.substr(1));

      current += (dir == 'R' ? 1 : -1) * value;
      current = current % 100;

      if (current == 0)
        zeroCount += 1;
    }
    return zeroCount;
  }

  int part2(const vector<string> &lines) {
    int current = 50;
    int res = 0;

    for (const string &line : lines) {
      const char dir = line[0];
      int shift = std::stoi(line.substr(1));

      if (dir == 'R') {
        current += shift;
        if (current >= 100) {
          res += current / 100;
        }
      } else { // L
        int prev = current;
        current -= shift;

        if (current <= 0) {
          res += (-current) / 100 + 1;
          if (prev == 0) {
            res -= 1;
          }
        }
      }

      current %= 100;
      if (current < 0)
        current += 100;
    }

    return res;
  }
};

}; // namespace aoc2025

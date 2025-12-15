#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "day1.h"

void printUsage(const std::string &appName) {
  std::cout << "Usage:" << std::endl;
  std::cout << "./" << appName << " <input>" << std::endl;
}

int main(int argc, char *argv[]) {
  std::string input;
  if (argc <= 1) {
    input = "day1/example.txt";
  } else {
    input = argv[1];
  }

  // if(argc != 2) {
  //   printUsage(argv[0]);
  //   return -1;
  // }

  std::fstream fs(input);

  std::vector<std::string> lines;
  std::string current;
  while (fs >> current)
    lines.push_back(current);

  auto solver = aoc2025::Day1();

  std::cout << "Running part1... " << std::endl;
  auto resPart1 = solver.part1(lines);
  std::cout << "Part1: " << resPart1 << std::endl;

  std::cout << "Running part2... " << std::endl;
  auto resPart2 = solver.part2(lines);
  std::cout << "Part2: " << resPart2 << std::endl;

  return 0;
}

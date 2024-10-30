#include "cpp.rs.h"

#include <iostream>

int main() {
  auto point = new_todo(1, "hello");
  std::cout << "Todo: (" << point.id << ", " << point.title << ")" << std::endl;

  return 0;
}

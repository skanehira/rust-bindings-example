#include "cpp.rs.h" // cxxが自動生成するヘッダーファイル
#include "cxx.h"

#include <iostream>

int main() {
  // Rustの関数を呼び出してPointを作成
  auto point = new_point(10, 20);
  std::cout << "Point: (" << point.x << ", " << point.y << ")" << std::endl;

  return 0;
}

#include "core.h"
#include <iostream>

int main() {
  // `new_todo`関数で`Todo`構造体を作成
  auto todo = new_todo(1, "Learn Rust");

  // `id`と`status`を出力
  std::cout << "Todo ID: " << get_id(*todo) << std::endl;

  // `status_as_int`でステータスを整数として出力
  std::cout << "Todo Status: " << status_as_int(*todo) << std::endl;

  return 0;
}

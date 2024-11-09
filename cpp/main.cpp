#include "core.h"
#include <iostream>

int main() {
  auto todo = new_todo(1, "Learn Rust");

  auto status = status_str(todo);

  std::cout << "Todo.id: " << todo->id << std::endl;
  std::cout << "Todo.title: " << todo->title << std::endl;
  std::cout << "Todo.status: " << status << std::endl;
  free_string(status);

  change_status(todo, TodoStatus::Completed);

  status = status_str(todo);
  std::cout << "Todo.status: " << status << std::endl;

  free_string(status);
  free_todo(todo);

  return 0;
}

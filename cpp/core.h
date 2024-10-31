#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct String;

struct Todo;

struct TodoStatus;

extern "C" {

Todo new_todo(int32_t id, String title);

TodoStatus status(const Todo *self);

bool completed(const Todo *self);

String title(const Todo *self);

void change_status(Todo *self, TodoStatus status);

void change_title(Todo *self, String title);

}  // extern "C"

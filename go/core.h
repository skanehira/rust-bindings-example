#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum TodoStatus {
  NotStarted,
  InProgress,
  Completed,
} TodoStatus;

typedef struct Todo {
  int32_t id;
  const char *title;
  enum TodoStatus status;
} Todo;

/**
 * # Safety
 */
struct Todo *new_todo(int32_t id, const char *title);

/**
 * # Safety
 */
void free_todo(struct Todo *o);

/**
 * # Safety
 */
void free_string(char *s);

char *status_str(const struct Todo *self);

bool completed(const struct Todo *self);

void change_status(struct Todo *self, enum TodoStatus status);

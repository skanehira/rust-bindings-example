from core import Todo, TodoStatus

todo = Todo(1, 'Buy milk')
todo.change_status(TodoStatus.Completed)
print(todo)

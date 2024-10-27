import core

todos = core.Todos()

todos.add(1, 'Buy milk')
todos.add(2, 'Buy bread')
todos.complete(1)
todos.add(3, 'Buy eggs')
todos.remove(2)

for todo in todos.list():
    print(todo)

try:
    todos.complete(10)
except ValueError as err:
    print(err)

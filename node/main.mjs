import { Todo, TodoStatus } from "./lib/core/index.js";

const todo = new Todo(1, "Learn WebAssembly");
console.log(todo);
todo.change_status(TodoStatus.Completed);
console.log(todo);

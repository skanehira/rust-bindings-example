import { Todos } from "./lib/core/index.js";

const todos = new Todos();
todos.add(1, "Learn JavaScript");
todos.add(2, "Learn Rust");
todos.complete(1);
todos.add(3, "Learn WebAssembly");
todos.complete(2);
todos.remove(2)

for (const todo of todos.list) {
  console.log(todo);
}

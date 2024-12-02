package main

/*
#cgo LDFLAGS: /work/lib/target/release/libcore.a
#include "./core.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	title := C.CString("Hello from Go")
	defer C.free(unsafe.Pointer(title))
	todo := C.new_todo(1, title)
	defer C.free_todo(todo)

	C.change_status(todo, C.Completed)

	status := C.status_str(todo)
	defer C.free_string(status)

	fmt.Printf("ID: %d, Title: %s, Status: %s\n", todo.id, C.GoString(todo.title), C.GoString(status))

	list := C.list(todo)
	defer C.free_ffivec(list)

	item := C.get_item_from_vec(list, 0)
	fmt.Printf("Item: %s\n", C.GoString(item))
	defer C.free_string(item)

	item = C.get_item_from_vec(list, 1)
	fmt.Printf("Item: %s\n", C.GoString(item))
	defer C.free_string(item)
}

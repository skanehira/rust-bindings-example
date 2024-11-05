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
	defer C.free(unsafe.Pointer(todo))

	C.change_status(todo, C.Completed)

	status := C.status_str(todo)
	defer C.free(unsafe.Pointer(status))

	fmt.Printf("ID: %d, Title: %s, Status: %s\n", todo.id, C.GoString(todo.title), C.GoString(status))
}

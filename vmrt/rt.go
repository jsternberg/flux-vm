package main

// #include "vm.h"
import "C"

import (
	"fmt"
	"runtime/cgo"

	"github.com/influxdata/flux/plan"
)

type ProcedureSpec struct {
	kind plan.ProcedureKind
}

func (s *ProcedureSpec) Kind() plan.ProcedureKind {
	return s.kind
}

func (s *ProcedureSpec) Copy() plan.ProcedureSpec {
	ns := *s
	return &ns
}

type Context struct {
	nodes  map[uint64]plan.Node
	yields []uint64
	nextID uint64
}

//export NewVMContext
func NewVMContext() C.uintptr_t {
	ctx := &Context{
		nodes: make(map[uint64]plan.Node),
	}
	h := cgo.NewHandle(ctx)
	return C.uintptr_t(h)
}

//export DestroyVMContext
func DestroyVMContext(handle C.uintptr_t) {
	h := cgo.Handle(handle)
	h.Delete()
}

//export CreateSource
func CreateSource(handle C.uintptr_t, name *C.char) C.uint64_t {
	h := cgo.Handle(handle)
	ctx := h.Value().(*Context)

	s := C.GoString(name)
	pn := plan.CreateLogicalNode(plan.NodeID(s), &ProcedureSpec{"gen.tables"})
	id := ctx.nextID
	ctx.nodes[id] = pn
	ctx.nextID++
	return C.uint64_t(id)
}

//export CreateTransformation
func CreateTransformation(handle C.uintptr_t, name *C.char, parent C.uint64_t) C.uint64_t {
	// Only a single parent is presently supported just for this demo.
	// We'll have to resolve how to send parameters in to create source and
	// create transformation.
	h := cgo.Handle(handle)
	ctx := h.Value().(*Context)

	s := C.GoString(name)
	pn := plan.CreateLogicalNode(plan.NodeID(s), &ProcedureSpec{"transform"})
	pn.AddPredecessors(ctx.nodes[uint64(parent)])
	id := ctx.nextID
	ctx.nodes[id] = pn
	ctx.nextID++
	return C.uint64_t(id)
}

//export Yield
func Yield(handle C.uintptr_t, nodeID C.uint64_t) {
	h := cgo.Handle(handle)
	ctx := h.Value().(*Context)
	ctx.yields = append(ctx.yields, uint64(nodeID))
}

//export ExecuteYields
func ExecuteYields(handle C.uintptr_t) {
	h := cgo.Handle(handle)
	ctx := h.Value().(*Context)
	if len(ctx.yields) == 0 {
		fmt.Println("no yields")
	}

	spec := &plan.Spec{
		Roots: make(map[plan.Node]struct{}),
	}
	for _, id := range ctx.yields {
		pn := ctx.nodes[id]
		spec.Roots[pn] = struct{}{}
	}
	fmt.Println(plan.Formatted(spec))
}

// Needed for c-archive.
func main() {}

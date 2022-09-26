#include "vm.h"
#include "vmrt.h"

int main() {
    uintptr_t ctx = NewVMContext();
    struct runtime_t *rt = runtime_create(ctx);

    struct value_t *v0 = value_int_new(4);
    runtime_print(rt, v0);

    uint64_t n0 = CreateSource(ctx, "gen.tables");
    uint64_t n1 = CreateTransformation(ctx, "transform", n0);
    Yield(ctx, n1);
    ExecuteYields(ctx);

    runtime_destroy(rt);
    DestroyVMContext(ctx);
    return 0;
}

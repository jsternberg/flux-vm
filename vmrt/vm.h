#ifndef _FLUX_VM_H
#define _FLUX_VM_H
#endif

#include <stdint.h>

struct value_t;
struct value_t *value_int_new(int64_t);
void value_free(struct value_t *);

struct runtime_t;
struct runtime_t *runtime_create(uintptr_t);
void runtime_destroy(struct runtime_t *);

void runtime_print(struct runtime_t *, struct value_t *);

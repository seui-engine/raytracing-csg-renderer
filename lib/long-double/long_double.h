#ifndef LONG_DOUBLE_H
#define LONG_DOUBLE_H

#include <stdbool.h>

typedef struct {
    unsigned char opaque[16];
} long_double_t;

long_double_t c_long_double_from_float(float a);
long_double_t c_long_double_from_double(double a);
float c_long_double_to_float(long_double_t a);
double c_long_double_to_double(long_double_t a);
long_double_t c_long_double_add(long_double_t a, long_double_t b);
long_double_t c_long_double_sub(long_double_t a, long_double_t b);
long_double_t c_long_double_mul(long_double_t a, long_double_t b);
long_double_t c_long_double_div(long_double_t a, long_double_t b);
long_double_t c_long_double_neg(long_double_t a);
long_double_t c_long_double_sqrt(long_double_t a);
long_double_t c_long_double_cbrt(long_double_t a);
long_double_t c_long_double_cos(long_double_t a);
long_double_t c_long_double_acos(long_double_t a);
long_double_t c_long_double_tan(long_double_t a);
long_double_t c_long_double_exp(long_double_t a);
long_double_t c_long_double_abs(long_double_t a);
bool c_long_double_isinf(long_double_t a);
bool c_long_double_isnan(long_double_t a);
bool c_long_double_gt(long_double_t a, long_double_t b);
bool c_long_double_gte(long_double_t a, long_double_t b);
bool c_long_double_lt(long_double_t a, long_double_t b);
bool c_long_double_lte(long_double_t a, long_double_t b);
bool c_long_double_eq(long_double_t a, long_double_t b);
bool c_long_double_ne(long_double_t a, long_double_t b);
long_double_t c_long_double_infinity(void);
long_double_t c_long_double_pi(void);
long_double_t c_long_double_pow(long_double_t a, long_double_t b);

#endif

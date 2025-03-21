#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "long_double.h"

static inline void c(void) {
    if (sizeof(long_double_t) < sizeof(long double)) {
        fputs("long double is not supported on this platform.", stderr);
        abort();
    }
}

static inline long double f(long_double_t t) {
    c();
    return *((long double *)&t);
}

static inline long_double_t t(long double f) {
    c();
    long_double_t result;
    *((long double *)&result) = f;
    return result;
}

long_double_t c_long_double_from_float(float a) {
    return t(a);
}

long_double_t c_long_double_from_double(double a) {
    return t(a);
}

float c_long_double_to_float(long_double_t a) {
    return (float)f(a);
}

double c_long_double_to_double(long_double_t a) {
    return (double)f(a);
}

long_double_t c_long_double_add(long_double_t a, long_double_t b) {
    return t(f(a) + f(b));
}

long_double_t c_long_double_sub(long_double_t a, long_double_t b) {
    return t(f(a) - f(b));
}

long_double_t c_long_double_mul(long_double_t a, long_double_t b) {
    return t(f(a) * f(b));
}

long_double_t c_long_double_div(long_double_t a, long_double_t b) {
    return t(f(a) / f(b));
}

long_double_t c_long_double_neg(long_double_t a) {
    return t(-f(a));
}

long_double_t c_long_double_sqrt(long_double_t a) {
    return t(sqrtl(f(a)));
}

long_double_t c_long_double_cbrt(long_double_t a) {
    return t(cbrtl(f(a)));
}

long_double_t c_long_double_cos(long_double_t a) {
    return t(cosl(f(a)));
}

long_double_t c_long_double_acos(long_double_t a) {
    return t(acosl(f(a)));
}

long_double_t c_long_double_tan(long_double_t a) {
    return t(tanl(f(a)));
}

long_double_t c_long_double_exp(long_double_t a) {
    return t(expl(f(a)));
}

long_double_t c_long_double_abs(long_double_t a) {
    return t(fabsl(f(a)));
}

bool c_long_double_isinf(long_double_t a) {
    return isinf(f(a));
}

bool c_long_double_isnan(long_double_t a) {
    return isnan(f(a));
}

bool c_long_double_gt(long_double_t a, long_double_t b) {
    return f(a) > f(b);
}

bool c_long_double_gte(long_double_t a, long_double_t b) {
    return f(a) >= f(b);
}

bool c_long_double_lt(long_double_t a, long_double_t b) {
    return f(a) < f(b);
}

bool c_long_double_lte(long_double_t a, long_double_t b) {
    return f(a) <= f(b);
}

bool c_long_double_eq(long_double_t a, long_double_t b) {
    return f(a) == f(b);
}

bool c_long_double_ne(long_double_t a, long_double_t b) {
    return f(a) != f(b);
}

long_double_t c_long_double_infinity(void) {
    return t(INFINITY);
}

long_double_t c_long_double_pi(void) {
    return t(3.141592653589793238l);
}

long_double_t c_long_double_pow(long_double_t a, long_double_t b) {
    return t(powl(f(a), f(b)));
}

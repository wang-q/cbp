#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <stdio.h>
#include <gsl/gsl_version.h>
#include <gsl/gsl_sf_bessel.h>
#include <gsl/gsl_statistics.h>

int main() {
    printf("GSL version: %s\n", GSL_VERSION);

    // Test Bessel function
    double x = 5.0;
    double expected = -0.17759677131433830434739701;
    double result = gsl_sf_bessel_J0(x);
    printf("J0(%g) = %.18f\n", x, result);

    // Test if the result is close enough to expected value
    double diff = result - expected;
    if (diff < 0) diff = -diff;

    // Test statistics functions
    double data[] = {1.0, 2.0, 3.0, 4.0, 5.0};
    double mean = gsl_stats_mean(data, 1, 5);
    double variance = gsl_stats_variance(data, 1, 5);

    printf("Mean of data: %g\n", mean);
    printf("Variance of data: %g\n", variance);

    // Verify results
    printf("Test %s\n",
           (diff < 1e-15 && mean == 3.0 && variance == 2.5)
           ? "PASSED" : "FAILED");

    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libgsl.a" \
        "$(cbp prefix lib)/libgslcblas.a" \
        -lm \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libgsl.a" \
        "$(cbp prefix lib)/libgslcblas.a" \
        -lm \
        -o test
    OUTPUT=$(./test)
fi

assert 'echo "${OUTPUT}" | grep -q "Test PASSED"' "Expected successful GSL computations"

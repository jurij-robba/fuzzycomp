# Fuzzy Comp

Comparison with margins.

## Justification

There are problems in engineering and elsewhere where similar enough numbers
count as equal. Take for example cruise control. We do not want to regulate
speed for every small change but when the speed deviates enough from desired we
either break or speed up.

Similarly, in finance, a candlestick that has the same open and close price is
called doji and signifies indecision in markets. Of course open and close need
only be close enough in real-world say 1/10 or less of the daily range.

Those are the sort of the problem the comparison functions in this library aim
to solve.

## Description

Notation follows Fortran's functions:

 * eq - Equal within margins
 * ne - Not equal despite margins
 * gt - Surely greater, even with margins
 * lt - Surely lesser, even with margins
 * ge - At least equal, within margins
 * le - At most equal, within margins

 Those functions can be used anywhere where we want to be just precise enough.

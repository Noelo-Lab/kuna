Recognize an implied integer upper extraction that covers the exact unchanged
prefix of a two-, four-, or eight-byte scalar. Also recognize the equivalent
right-shift/truncate form introduced by cast insertion. Print the original
source masked to the upper prefix, OR the lower expression masked to its
machine width. Cast operands to the unsigned result width. Decline named
upper values, noninteger values, enums, unrelated joins and wider scalars.
The replaced suffix must have a native one-, two-, or four-byte integer width.

Expose partialconcat on/off, retain the IR and prototypes, and ship on only
with unchanged existing corpus assertions and a measured speed delta within
5%. Cover the witness, a 64-bit form and an unrelated join in the stage test.

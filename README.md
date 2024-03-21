# kernel-tuning-base
Lil Rust project to measure network latencies. 

Run on one server that is the control and another server that is being tuned. Save the data to "control" & "tuned" folders then run the python analyze.py file for comparison.

// =-= Experiments =-= //

// 1. Slow traffic, w/ no data\
// 2. Slow traffic, w/ data\
// 3. Burst traffic, w/ no data\
// 4. Burst traffic, w/ data\
// 5. Consistent traffic, w/ no data\
// 6. Consistent traffic, w/ data\
// 7. Slow traffic, w/ large data\
// 8. Burst traffic, w/ large data\
// 9. Consistent traffic, w/ large data\
// =---------------------------------------------------------= //\
// =-= Latencies =-= //\
// 50us = burst\
// 15ms = consistent\
// 50ms = slow\
// =-= Message Sizes =-= //\
// No Data: 0KB\
// w/ Data: 32bytes\
// Large Data: 256bytes\

# kernel-tuning-base
Lil Rust project to aid in the objective of tuning the kernel's network stack

#

### Setup:
1. Run on the control server
2. On the server being tuned, change the kernel and run it
3. Move their respective results (found in "latency") to a computer with GUI support under the "tuned_control" folder, saving the results in their respective folders.
4. Run python3 analyze.py to compare their results. Change the "threshold" variable to cut off latencies if you'd like (helps to zoom in and out)
5. Iterate #2 after deleting the results in tuned's "latency" folder

#

### Experiments:
- Slow traffic, w/ no data
- Slow traffic, w/ data
- Slow traffic, w/ large data
- Burst traffic, w/ no data
- Burst traffic, w/ data
- Burst traffic, w/ large data
- Consistent traffic, w/ no data
- Consistent traffic, w/ data
- Consistent traffic, w/ large data

### Push Frequency:
- Slow = 50ms (sample: 2,000x)
- Burst = 50us (sample: 15,000x)
- Consistent = 15ms (sample: 6,000x)
- Sample size was chosen to be dynamic based on the push frequency to limit how long the experiment runs

### Message Sizes:
- No Data: 0 bytes
- w/ Data: 32 bytes
- Large Data: 256 bytes

#

### Notes
- Subsequent runs will not delete data in the latency folder. Meaning, feel free to stack up sample sizes by running multiple times
- Burst w/ LargeData is likely to show nil improvement since the receiving side takes too long to process the request before receiving another update. It's outside my specific scope so I kept the infra how it is but if anyone would like to optimize it, please feel free! Can DM me on twitter / x @Dub0x3A too if you'd like


#

### Disclaimer: 
`Best practices of code were not implemented` here so pls don't use this to gain inspiration for your own projects. This is a simple application that aims to capture the relative change in latency between a tuned and non-tuned kernel, as such, the code was not optimized as each server will run an identical copy. `The absolute latency values are not important, but the relative change between the two is. `
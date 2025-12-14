## Advent of code 2024
My solutions for the 2024 Advent of Code event. All solutions are written in Rust, and are of questionable quality since it's my first time using the language.
The code is reasonably performant however and solves all problems in ~ 200 ms (see benchmarks below).

### Benchmarks
Results from `./run.sh` using a 2024 M4 MacBook Pro (16GB)
```
----------------------------------------
Day        Time (ms)       Cumulative (ms)
----------------------------------------
1          0.26            0.26
2          0.93            1.19
3          0.13            1.32
4          0.75            2.07
5          3.07            5.14
6          20.39           25.53
7          6.24            31.76
8          0.22            31.98
9          1.75            33.74
10         0.40            34.14
11         6.97            41.11
12         2.12            43.23
13         0.35            43.58
14         40.20           83.78
15         2.00            85.78
16         12.31           98.09
17         0.02            98.11
18         0.31            98.42
19         1.00            99.42
20         4.89            104.31
21         5.81            110.12
22         9.15            119.26
23         16.67           135.94
24         0.17            136.10
25         0.02            136.13
----------------------------------------
Total                      136.13
```

### Requirements
- Rust (I used 1.79.0 but other versions should work)
- The `aocd` python library (`pip install aocd`)
- A session token from AoC (`export AOC_SESSION=<your session token>`)

#### Inputs
AoC problems require (personal) AoC inputs, and the AoC creator has requested that they not be shared publicly (hence why they are not in this repo).
Instead, use the `get_inputs.py` script to download all inputs into the `inputs` directory prior to running the rust code.
The script assumes you have exported your session token as an environment variable.

### Running solutions
Run either
- `./run.sh` to run all days and get a benchark for the runtime
or
- `./run.sh dayx` to run a specific day (e.g. `./run.sh day1`)


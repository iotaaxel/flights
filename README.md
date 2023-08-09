# flights

## Overview
In order to determine the flight path of a person, we must sort through all of their flight records. Accept a request that includes a list of flights, which are defined by a source and destination airport code. These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

## Json Structure
```
[["SFO", "EWR"]]                              => ["SFO", "EWR"]
[["ATL", "EWR"], ["SFO", "ATL"]]              => ["SFO", "EWR"]
[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]] => ["SFO", "EWR"]
```

## Considerations
The current solution is adequate but it would be cool to make a more efficient solution. If invested, it would be nice to make a visualization and show a graph-based solution.
Run the program with `cargo run` and run the tests with `cargo test`. 

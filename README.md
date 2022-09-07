# Condition

On an infinite grid there is an ant that could move one cell at a time:
up (x,y+1)
down(x,y-1)
left (x-1,y)
right (x+1,y)

Cell with sum of digits of it's coordinates exceeds 25 are not allowed to be visited.

Example:
cell (59, 79) is out of reach because 5+9+7+9=30

How many cells including initial cell could ant visit if his initial position is (1000, 1000)?

The answer is **148848**

The project is run by cargo run

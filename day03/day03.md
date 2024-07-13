The solution is a naive one. First I read the input and parse it into domain data structure.

Each claim is processed such that every coordinate in the fabric is an entry in a Hashmap. The key is the coordinate and the value is a vector of references to each claim that covers that coordinate.

For part 1, I just count the number of coordinates that have more than one claim covering it.

For part 2, I compute an intermediate Hashmap that maps each claim to the set of uncontested coordinates that it covers. Then I find the claims that have all their coordinates uncontested.

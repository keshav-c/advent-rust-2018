Part 1 was easy. You get a clean file with integers in each line. You just need to read the file, parse the integers, and sum them up.

In Part 2 we are asked to consider a scenario where the numbers in the input keep repeating. We need to find the first delta that repeats.

Having read about Iterators just recently, I thought to use that. My idea was to create a custom Iterator that would keep track of the changing delta in a HashSet. If inserting into the HashSet returns false, we have found the first repeating delta.

A simple for loop would just keep iterating through the set of frequencies till we find the repeating delta.

Initially I added all the state to the struct that wraps the frequency list, but the struct gets consumed in the for loop. So I moved the iteration state to another IteratorStruct which would actually implement the Iterator trait. The wrapping struct would implement IntoIterator which returns the IteratorStruct. This was the wrapping struct is left intact and can be queried for the result after the iteration is done.

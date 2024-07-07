The approach I am taking leads to creating types that describe the problem domain and then define or implement traits on them. Then to get to the solution, simple code like loops would just work.

The problem data is just a file with data points in each line. I implement a From trait for the struct that will hold the parsed input. And then the methods on the struct would be used to solve the problem.

One new thing I did here is to use the zip method on the iterator. This required me to define an IntoIterator trait for the data struct because that is what this method expects.

For the part 2, I have just done a brute force search through the data.

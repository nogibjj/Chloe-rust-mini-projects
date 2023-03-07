# File Compression Program
The Rust file compression project is a simple command-line tool that compresses a file using the Gzip compression algorithm.

When you run the program, it expects two command-line arguments: the path to the input file and the path to the output file. It reads the contents of the input file, compresses them using Gzip, and writes the compressed data to the output file.

Here's a brief overview of how the program works:

1. The main function first checks that the program was called with exactly two command-line arguments. If there are not exactly two arguments, it prints an error message and exits.

2. If there are two arguments, the function opens the input file specified by the first argument using the File::open method and wraps it in a BufReader to allow efficient reading of the file.

3. The output file is created using the second command-line argument with the File::create method. The unwrap method is called on the File::create call to panic and terminate the program if there was an error creating the output file.

4. A GzEncoder is created using the output file and a default Compression level.

5. The copy method is used to read the contents of the input file and write them to the encoder. This compresses the data using the Gzip algorithm.

6. The encoder.finish() method is called to flush any remaining data and finalize the compressed output. The compressed data is then written to the output file.

7. Finally, some metadata is printed to the console, including the size of the original file and the compressed file, as well as the elapsed time for the compression process.

# How to use
- To build and run the program, 
```
cargo run <source filename> <compressed file name>

```
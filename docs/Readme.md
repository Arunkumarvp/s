## Run the Mojo NumPy example

Install Mojo and NumPy, then run the standalone Mojo source file:

```bash
python -m pip install max
mojo run "pymojo_programming practical/code/main.mojo"
```

The program imports NumPy through Mojo's Python interoperability layer and
prints the array `[1 2 3]`.
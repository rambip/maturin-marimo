value=$(cat src/magic_number.txt)
# bump value
echo $((value + 1)) > src/magic_number.txt
# recompile
.venv/bin/maturin develop
# test function
.venv/bin/python3 -c "import maturin_marimo; print(maturin_marimo.magic_number())"
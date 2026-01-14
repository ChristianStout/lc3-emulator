# LC-3 Emulator
---------------

This is a command line tool that runs LC-3 assembly language as defined by the LC-3 [Instruction Set Architecture](https://acsa.ustc.edu.cn/ics/download/lc3/lc3-handbook.pdf) (ISA).

This project comprises both a command-line tool, and also a web front end. Both
share the same underlying LC-3 implementation (found in `/lc3`).

## IMPORTANT NOTICE
This is still an extremely early release, and the web front-end is not complete.
The command-line tool seems to be fully function (other than privileged mode).

As of now, this is an emulator. Soon, I plan to make this a full simulator by 
implementing the OS and getting rid of trap magic.

## Setup
To use the web front end, just click [here](https://christianstout.github.io/lc3-emulator)!

# Command-line Runner
Clone the repo and enter the repo:
```bash
git clone https://github.com/ChristianStout/lc3-emulator.git && \
cd lc3-emulator
```

Then to compile the cli tool, run:
```bash
cargo build --release -p lc3-cli
```

Navigate to the binary from the repo root:
```bash
cd target/release
```

## Using the command-line
To run an assemble and run an LC-3 assembly file, run:
```bash
./lc3 <FILE_PATH>
```

You can also emit the binary file as `out.bin` if you run:
```bash
./lc3 <FILE_PATH> --emit-binary
```

# Goals and roadmap
The following are goals that need to be met for each package/library

### Cli
[ ] Implement running from a binary file with `--binary` flag

### LC-3 Backend
[ ] Write OS in assembly and load into memory before file
  [ ] Allow users to write their own OS by using a `.ORIG` in the privileged zone in memory
[ ] Implement RTI
[ ] Implement IR
[ ] Have PSR reflect the internal state of the machine
[ ] Make `Instruction.exe()` return an optional memory address that may have been modified (for front end to update the memory value)

### Web Front-End
[ ] Update register view on step
[ ] Fix side panel
  [ ] Add memory view
  [ ] make console/memory view's height dynamic to the height of the panel
  [ ] Properly position clear input/output button
  [ ] Make a way to view the input stream + count
[ ] Reimplement syntax highlighting into the editor
[ ] Add Github link button
[ ] Switch to Yew/React?

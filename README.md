# Chip-8 Emulator
A Chip-8 interpreter written in Rust, this emulator currently supports most modern chip-8 files, although there may yet be some exceptions that do not work well on this emulator.

## Instruction Set
As the original Chip-8 contained 35 instructions, this one currently supports around 33-34, it does not support instructions like 0xFX71 (which worked for sending data in VX to RS485 port), 
given the purpose of this project, i felt there was little meaning in adding hardware-level instructions.

## Further Additions
Currently, the program possesses no UI and the file name needs to be hard-coded into the program before running it, however I intend on adding a TUI to it soon plus optimizing it further so that 
it can work with the files it currently can't.

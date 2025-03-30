# Dielectric Dreams

A Fantasy Computer/Console 

## Plans

This project will combine everything that i like to fiddle with i.e. Architecture of CPUs, Operating Systems, Compilers and probably other stuff
my rough plan is (if i will ever finish it...):

1. Create a 32Bit Emulation of a CPU that is entirely my own.
2. Create a OS for it.
3. Make Tools available, compilers, sprite editors (think of picotron)
4. Make it runnable on a FPGA

## ISA

The ISA is in a bad shape right now, i want to clean it up and make it non orthogonal as i learned that orthogonality
in opcodes is a think of the past. Also i need to move the Opcodes in the High Bits of the ISA.

## Emulation

The Emulation Layer will probably be completely rewritten due to the changes on the ISA, i hope to make it easier to
read in the long run.

## Assembler

The Assembler should also be changed, and i need it as early as possible, because testing these things manually and assemble
from scratch is a nightmare which i don't want to do again...

# Projects

# Dielectric Cpu

This is the project that contains the actual cpu emulation it will grow over time

# Dielectric ISA generator

Helpful tool in generating all possible opcodes quickly

# assembler

The assembler for my ISA


## Resources

### Verilog
[Nandland](https://nandland.com/learn-verilog/)
[chipverify](https://www.chipverify.com/verilog/verilog-tutorial)
[fpga4fun](https://www.fpga4fun.com/)
[MIT Courseware 6.111](http://web.mit.edu/6.111/volume2/www/f2019/index.html)
[EDX Building a RISC-V Core](https://www.edx.org/learn/design/the-linux-foundation-building-a-risc-v-cpu-core)
[FreeRange VHDL](https://www.isy.liu.se/edu/kurs/TSEA83/kursmaterial/vhdl/free_range_vhdl_2019.pdf)

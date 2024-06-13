# Memory

The user has 128MB of Memory to play with! A bit less as we need to reserve some of it for internal purposes.

Video Memory for FrameBuffer Mode: 480 * 270 * (24Bit Depth)
Video Memory for ColorTable Mode: 480 * 270 * (8Bit);

## Memory Map
```
0x00000000 - 0x00002000 (8 KB) BIOS
0x00002000 - 0x00002400 (1 KB) Jumptable Interrupts
0x00002400 - 0x00080900 (526KB) Video Memory
...
```
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include <unistd.h>
#include <errno.h>
#include "types.h"

void PrintFatalError(const char *msg)
{
    fprintf(stderr, "ERROR: %s\n", msg);
    exit(1);
}

// FIXME: Possible Buffer overrun when the input file exceeds 4096 bytes!
// TODO: Remove global buffer declaration, move into some kind of string structure!
// TODO: This is a temporary read buffer it will suffice for now but later we should allocate memory or
//       have some kind of pooled memory arena!
u8 buffer[4096] = {0};
usize bufferLength = 0;

u8 *ReadSourceFile(const char *filePath)
{
    FILE *sourceFile = fopen(filePath, "r");
    if (!sourceFile)
    {
        PrintFatalError("cannot open file.\n");
    }

    // NOTE: The last byte should be the zero byte so we only read up to 4095 bytes
    //       if we read more than that horrible things would happen, quite probably
    //       we would run into this function and happily overwrite some bytes...
    bufferLength = fread(&buffer, 1, 4095, sourceFile);
    // TODO: Return the source here!
    return buffer;
}

void WriteAssembly(FILE *output, const char *assembly)
{
    fprintf(output, "%s\n", assembly);
}

FILE *OpenOutputFile(const char *outputPath)
{
    FILE *outputFile = fopen(outputPath, "w+");
    if (!outputFile)
    {
        PrintFatalError("cannot open output file.\n");
    }

    return outputFile;
}

void SimulationMode(const char *filePath)
{
    ReadSourceFile(filePath);
    fprintf(stdout, "%s:%ld\n", buffer, bufferLength);

    u8 *token = (u8 *)strtok((char *)buffer, "() ");

    u32 returnValue = 0;
    while (token)
    {
        // NOTE: For now we assume that we can only have values, so we take the last value and use it as the application return value
        returnValue = atoi((char *)token);
        token = (u8 *)strtok(NULL, "() ");
    }

    // FIXME: Not the way we would return a value but okay for now...
    printf("Returning with value %d\n", returnValue);
}

void CompilationMode(const char *filePath, const char *outputPath)
{
    u8 *source = ReadSourceFile(filePath);
    FILE *asmOutput = OpenOutputFile(outputPath);

    // These are the magic NASM incantations!
    WriteAssembly(asmOutput, "section .text");
    WriteAssembly(asmOutput, "global _start");
    WriteAssembly(asmOutput, "_start:");
    WriteAssembly(asmOutput, "\t;; Exit");
    WriteAssembly(asmOutput, "\tmov rax, 60");

    u8 *token = (u8 *)strtok((char *)source, "() ");

    while (token)
    {
        u32 value = atoi(token);
        const char op[256] = {0}; // FIXME: This is a recepie for desaster!
        // FIXME: Clang is not amused about the next line...
        sprintf(op, "\tmov rdi,%d", value);
        WriteAssembly(asmOutput, op);
        token = (u8 *)strtok(NULL, "() ");
    }

    WriteAssembly(asmOutput, "\tsyscall");

    // NOTE: If we don't close the file nasm will have some issues compiling the file...
    fclose(asmOutput);

    // NOTE: System should be replaced with a safer version!
    if (system("nasm -felf64 output.asm -o output.o") == -1)
    {
        fprintf(stderr, "cannot assemble the file %s", strerror(errno));
    }

    if (system("ld -o output output.o") == -1)
    {
        fprintf(stderr, "cannot link the file %s", strerror(errno));
    }

    system("rm output.o");

    return;
}

int main(int argc, char const *argv[])
{
    if (argc <= 1)
    {
        fprintf(stderr, "USAGE <MODE> <file>\nMODE:\n\tsim - simulate the source file\n\tcomp - compile the source file\n\n");
        exit(1);
    }

    if (strcmp(argv[1], "sim") == 0)
    {
        SimulationMode(argv[2]);
    }
    else if (strcmp(argv[1], "comp") == 0)
    {
        CompilationMode(argv[2], "output.asm");
    }
    else
    {
        PrintFatalError("unknown Mode!\n");
    }

    return 0;
}

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include <string.h>
#include <stdbool.h>
#include <unistd.h>
#include <errno.h>
#include "types.h"

#include "lexer.h"

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
    // TODO: Replace the strtok call with our lexer!
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

void Parser(FILE *asmOutput, Lexer *lexer)
{
    Token token = NextToken(lexer);

    while (token.tt != TOKEN_TYPE_EOF)
    {
        if (token.tt == TOKEN_TYPE_OPEN_PAREN)
        {
            Parser(asmOutput, lexer);
            token = NextToken(lexer);
        }
        else if (token.tt == TOKEN_TYPE_CLOSE_PAREN)
        {
            return;
        }
        else if (token.tt == TOKEN_TYPE_DUMP)
        {
            // NOTE: Right now we assume that the value to dump is __always__ in rdi
            token = NextToken(lexer);

            if (token.tt == TOKEN_TYPE_OPEN_PAREN)
            {
                Parser(asmOutput, lexer);
            }

            WriteAssembly(asmOutput, "\t;; CALL DUMP");
            // WriteAssembly(asmOutput, op);
            WriteAssembly(asmOutput, "\tcall dump");
        }
        else if (strncmp((const char *)token.tokenStart, "+", token.length) == 0)
        {
            token = NextToken(lexer);

            char op[256] = {0}; // FIXME: ...

            WriteAssembly(asmOutput, "\t;; ADDITION");
            while (token.tt == TOKEN_TYPE_VALUE)
            {
                // TODO: OPTIMIZATION
                //       We could make this code smarter aggregating all given constant values
                //       and just move them into rdi!
                u32 value = strtol((const char *)token.tokenStart, NULL, 10);
                sprintf(op, "\tadd rdi,%d", value);

                WriteAssembly(asmOutput, op);

                token = NextToken(lexer);
            }
        }
        else if (strncmp((const char *)token.tokenStart, "-", token.length) == 0)
        {
            token = NextToken(lexer);

            char op[256] = {0}; // FIXME: Again this should be refactored into something useable!

            WriteAssembly(asmOutput, "\t;; SUBTRACTION");

            u32 value = strtol((const char *)token.tokenStart, NULL, 10);
            sprintf(op, "\tmov rax,%d", value);
            WriteAssembly(asmOutput, op);

            token = NextToken(lexer);
            while (token.tt == TOKEN_TYPE_VALUE)
            {
                u32 value = strtol((const char *)token.tokenStart, NULL, 10);
                sprintf(op, "\tsub rax, %d", value);
                WriteAssembly(asmOutput, op);

                token = NextToken(lexer);
            }
        }
        else
        {
            u32 value = strtol((const char *)token.tokenStart, NULL, 10);
            char op[256] = {0}; // FIXME: This is a recepie for desaster!
            // FIXME: Clang is not amused about the next line...
            sprintf(op, "\tmov rdi,%d", value);
            WriteAssembly(asmOutput, op);
            token = NextToken(lexer);
        }

        token = NextToken(lexer);
    }
}

void CompilationMode(const char *filePath, const char *outputPath)
{
    u8 *source = ReadSourceFile(filePath);

    FILE *asmOutput = OpenOutputFile(outputPath);

    // These are the magic NASM incantations!
    WriteAssembly(asmOutput, "section .text");
    WriteAssembly(asmOutput, "global _start");

    //---- UTILTIY FUNCTIONS ----
    // This assembly was taken at verbatim from godbolt
    WriteAssembly(asmOutput, ";; Godbolt Dump Routine");
    WriteAssembly(asmOutput, "dump:");
    WriteAssembly(asmOutput, "\tsub     rsp, 40");
    WriteAssembly(asmOutput, "\tpxor    xmm0, xmm0");
    WriteAssembly(asmOutput, "\tmov     r8, -3689348814741910323");
    WriteAssembly(asmOutput, "\tmovaps  [rsp+16], xmm0");
    WriteAssembly(asmOutput, "\tlea     rcx, [rsp+30]");
    WriteAssembly(asmOutput, "\tmov     BYTE [rsp+31], 10");
    WriteAssembly(asmOutput, "\tmovaps  [rsp], xmm0");
    WriteAssembly(asmOutput, ".L2:");
    WriteAssembly(asmOutput, "\tmov     rax, rdi");
    WriteAssembly(asmOutput, "\tmul     r8");
    WriteAssembly(asmOutput, "\tmov     rax, rdi");
    WriteAssembly(asmOutput, "\tshr     rdx, 3");
    WriteAssembly(asmOutput, "\tlea     rsi, [rdx+rdx*4]");
    WriteAssembly(asmOutput, "\tadd     rsi, rsi");
    WriteAssembly(asmOutput, "\tsub     rax, rsi");
    WriteAssembly(asmOutput, "\tmov     rsi, rcx");
    WriteAssembly(asmOutput, "\tsub     rcx, 1");
    WriteAssembly(asmOutput, "\tadd     eax, 48");
    WriteAssembly(asmOutput, "\tmov     [rcx+1], al");
    WriteAssembly(asmOutput, "\tmov     rax, rdi");
    WriteAssembly(asmOutput, "\tmov     rdi, rdx");
    WriteAssembly(asmOutput, "\tcmp     rax, 9");
    WriteAssembly(asmOutput, "\tja      .L2");
    WriteAssembly(asmOutput, "\tmov     rax, 1");
    WriteAssembly(asmOutput, "\tlea     rdx, [rsp+32]");
    WriteAssembly(asmOutput, "\tmov     edi, 1");
    WriteAssembly(asmOutput, "\tsub     rdx, rsi");
    WriteAssembly(asmOutput, "\tsyscall");
    WriteAssembly(asmOutput, "\tadd     rsp, 40");
    WriteAssembly(asmOutput, "\tret");
    //---- END OF UTILITY BLOCK ----

    WriteAssembly(asmOutput, "_start:");

    Lexer lexer = NewLexer(source);
    Parser(asmOutput, &lexer);

    // TODO: Great now this all a position dependent mess :D
    WriteAssembly(asmOutput, "\t;; Exit");
    WriteAssembly(asmOutput, "\tmov rax, 60");
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

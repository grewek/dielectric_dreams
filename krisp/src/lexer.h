#pragma once
#include "types.h"

extern u8 tempBuffer[4096];

typedef enum
{
    TOKEN_TYPE_OPEN_PAREN,
    TOKEN_TYPE_CLOSE_PAREN,
    TOKEN_TYPE_VALUE,
    TOKEN_TYPE_PLUS,
    TOKEN_TYPE_MINUS,
    TOKEN_TYPE_DUMP,
    TOKEN_TYPE_EOF,

} TokenType;

typedef struct
{
    const u8 *source;
} Lexer;

typedef struct
{
    TokenType tt;
    const u8 *tokenStart;
    const u8 *tokenEnd;
    usize length;
} Token;

Lexer NewLexer(const char *source);
Token NextToken(Lexer *lexer);
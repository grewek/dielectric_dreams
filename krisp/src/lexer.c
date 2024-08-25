#include <stdio.h>
#include <assert.h>
#include <stdbool.h>

#include "lexer.h"

Lexer NewLexer(const char *source)
{
    Lexer result = {.source = source};
    return result;
}

bool IsDigit(u8 digit)
{
    if ((digit >= '0' && digit <= '9'))
    {
        return true;
    }

    return false;
}

bool IsAlpha(u8 letter)
{
    if ((letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z'))
    {
        return true;
    }

    return false;
}

Token DigestIdentifier(Lexer *lexer)
{
    u8 *start = lexer->source;
    u8 *end = lexer->source;

    while (*lexer->source && IsAlpha(*lexer->source))
    {
        end += 1;
        *lexer->source++;
    }

    // TODO: For now we can only match dump tokens all the other identifiers are lost in time..
    Token result = {.tt = TOKEN_TYPE_DUMP, .tokenStart = start, .tokenEnd = end, .length = end - start};

    return result;
}

Token DigestValue(Lexer *lexer)
{
    u8 *start = lexer->source;
    u8 *end = lexer->source;
    while (*lexer->source && IsDigit(*lexer->source))
    {
        end += 1;
        *lexer->source++;
    }

    assert(start != end);

    Token result = {.tt = TOKEN_TYPE_VALUE,
                    .tokenStart = start,
                    .tokenEnd = end,
                    .length = end - start};

    return result;
}

Token NextToken(Lexer *lexer)
{
    while (*lexer->source)
    {
        if (*lexer->source == '(')
        {
            Token result = {.tt = TOKEN_TYPE_OPEN_PAREN, .tokenStart = lexer->source, .tokenEnd = lexer->source, .length = 1};
            *lexer->source++;
            return result;
        }
        else if (*lexer->source == ')')
        {
            Token result = {.tt = TOKEN_TYPE_CLOSE_PAREN, .tokenStart = lexer->source, .tokenEnd = lexer->source, .length = 1};
            *lexer->source++;
            return result;
        }
        else if (*lexer->source == '+')
        {
            Token result = {.tt = TOKEN_TYPE_PLUS, .tokenStart = lexer->source, .tokenEnd = lexer->source, .length = 1};
            *lexer->source++;
            return result;
        }
        else if (*lexer->source == '_' || IsAlpha(*lexer->source))
        {
            return DigestIdentifier(lexer);
        }
        else if (IsDigit(*lexer->source))
        {
            return DigestValue(lexer);
        }

        *lexer->source++;
    }

    assert(*lexer->source == NULL);
    Token result = {.tt = TOKEN_TYPE_EOF, .tokenStart = lexer->source, .tokenEnd = lexer->source, .length = 1};
    return result;
}
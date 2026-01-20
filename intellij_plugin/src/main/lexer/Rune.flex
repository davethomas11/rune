package com.vectrune;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.TokenType;
import com.intellij.psi.tree.IElementType;
import static com.vectrune.RuneTypes.*;

%%

%public
%class _RuneLexer
%implements FlexLexer
%unicode
%function advance
%type IElementType

LineTerminator = \r|\n|\r\n
WHITESPACE = {LineTerminator} | [ \t\f]
COMMENT = "#" [^\r\n]*
SECTION = "@" [A-Za-z0-9_/]+
KEY = [a-zA-Z_][a-zA-Z0-9_]*
EQUALS = "="
VALUE = [^\r\n# ]+

%%

{WHITESPACE}    { return TokenType.WHITE_SPACE; }
{COMMENT}       { return COMMENT; }
{SECTION}       { return SECTION; }
{KEY}           { return KEY; }
"="             { return EQUALS; }
{VALUE}         { return VALUE; }
.               { return TokenType.BAD_CHARACTER; }

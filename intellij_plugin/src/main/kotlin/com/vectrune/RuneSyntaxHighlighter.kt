package com.vectrune

import com.intellij.lexer.Lexer
import com.intellij.openapi.editor.DefaultLanguageHighlighterColors
import com.intellij.openapi.editor.colors.TextAttributesKey
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase
import com.intellij.psi.tree.IElementType

class RuneSyntaxHighlighter : SyntaxHighlighterBase() {
    override fun getHighlightingLexer(): Lexer = RuneLexerAdapter()

    override fun getTokenHighlights(tokenType: IElementType): Array<TextAttributesKey> {
        return when (tokenType) {
            RuneTypes.SECTION -> arrayOf(DefaultLanguageHighlighterColors.FUNCTION_DECLARATION)
            RuneTypes.KEY -> arrayOf(DefaultLanguageHighlighterColors.KEYWORD)
            RuneTypes.VALUE -> arrayOf(DefaultLanguageHighlighterColors.STRING)
            RuneTypes.COMMENT -> arrayOf(DefaultLanguageHighlighterColors.LINE_COMMENT)
            else -> emptyArray()
        }
    }
}
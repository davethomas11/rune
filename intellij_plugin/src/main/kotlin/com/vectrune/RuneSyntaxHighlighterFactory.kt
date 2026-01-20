package com.vectrune

import com.intellij.openapi.fileTypes.SingleLazyInstanceSyntaxHighlighterFactory

class RuneSyntaxHighlighterFactory : SingleLazyInstanceSyntaxHighlighterFactory() {
    override fun createHighlighter() = RuneSyntaxHighlighter()
}
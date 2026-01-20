package com.vectrune

import com.intellij.openapi.fileTypes.LanguageFileType
import javax.swing.Icon

class RuneFileType private constructor() : LanguageFileType(RuneLanguage) {
    companion object {
        @JvmField // This makes it accessible as RuneFileType.INSTANCE in Java/Kotlin
        val INSTANCE = RuneFileType()
    }

    override fun getName() = "Rune File"
    override fun getDescription() = "Vectrune configuration file"
    override fun getDefaultExtension() = "rune"
    override fun getIcon(): Icon? = RuneIcons.FILE
}
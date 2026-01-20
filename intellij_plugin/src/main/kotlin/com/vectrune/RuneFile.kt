package com.vectrune

import com.intellij.extapi.psi.PsiFileBase
import com.intellij.openapi.fileTypes.FileType
import com.intellij.psi.FileViewProvider

class RuneFile(viewProvider: FileViewProvider) : PsiFileBase(viewProvider, RuneLanguage) {
    override fun getFileType(): FileType = RuneFileType.INSTANCE
    override fun toString(): String = "Rune File"
}
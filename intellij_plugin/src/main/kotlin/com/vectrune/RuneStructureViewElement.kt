package com.vectrune

import com.intellij.ide.structureView.StructureViewTreeElement
import com.intellij.ide.util.treeView.smartTree.TreeElement
import com.intellij.navigation.ItemPresentation
import com.intellij.psi.NavigatablePsiElement
import com.intellij.psi.util.PsiTreeUtil
import com.vectrune.RuneFile
import com.vectrune.psi.RuneProperty
import com.vectrune.psi.RuneSectionGroup

class RuneStructureViewElement(private val element: NavigatablePsiElement) : StructureViewTreeElement {
    override fun getValue(): Any = element

    override fun navigate(requestFocus: Boolean) = element.navigate(requestFocus)

    override fun canNavigate(): Boolean = element.canNavigate()

    override fun canNavigateToSource(): Boolean = element.canNavigateToSource()

    override fun getPresentation(): ItemPresentation = element.presentation!!

    override fun getChildren(): Array<TreeElement> {
        return when (element) {
            is RuneFile -> {
                // Find all top-level sections
                val sections = PsiTreeUtil.getChildrenOfType(element, RuneSectionGroup::class.java)
                sections?.map { RuneStructureViewElement(it as NavigatablePsiElement) }?.toTypedArray() ?: emptyArray()
            }
            is RuneSectionGroup -> {
                // Find properties inside a section
                val properties = PsiTreeUtil.getChildrenOfType(element, RuneProperty::class.java)
                properties?.map { RuneStructureViewElement(it as NavigatablePsiElement) }?.toTypedArray() ?: emptyArray()
            }
            else -> emptyArray()
        }
    }
}
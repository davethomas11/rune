package com.vectrune.psi.impl

import com.intellij.navigation.ItemPresentation
import com.intellij.icons.AllIcons
import com.vectrune.psi.RuneProperty
import com.vectrune.psi.RuneSectionGroup
import javax.swing.Icon

class RunePsiImplUtil {
    companion object {
        @JvmStatic
        fun getPresentation(element: RuneSectionGroup): ItemPresentation {
            return object : ItemPresentation {
                override fun getPresentableText(): String? = element.firstChild.text
                override fun getLocationString(): String? = null
                override fun getIcon(unused: Boolean): Icon = AllIcons.Nodes.Package
            }
        }

        @JvmStatic
        fun getPresentation(element: RuneProperty): ItemPresentation {
            return object : ItemPresentation {
                override fun getPresentableText(): String? = element.text
                override fun getLocationString(): String? = null
                override fun getIcon(unused: Boolean): Icon = AllIcons.Nodes.Property
            }
        }
    }
}
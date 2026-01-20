// This is a generated file. Not intended for manual editing.
package com.vectrune;

import com.intellij.psi.tree.IElementType;
import com.intellij.psi.PsiElement;
import com.intellij.lang.ASTNode;
import com.vectrune.psi.impl.*;

public interface RuneTypes {

  IElementType PROPERTY = new RuneTokenType("PROPERTY");
  IElementType SECTION_GROUP = new RuneTokenType("SECTION_GROUP");

  IElementType COMMENT = new RuneTokenType("COMMENT");
  IElementType EQUALS = new RuneTokenType("=");
  IElementType KEY = new RuneTokenType("KEY");
  IElementType SECTION = new RuneTokenType("SECTION");
  IElementType VALUE = new RuneTokenType("VALUE");

  class Factory {
    public static PsiElement createElement(ASTNode node) {
      IElementType type = node.getElementType();
      if (type == PROPERTY) {
        return new RunePropertyImpl(node);
      }
      else if (type == SECTION_GROUP) {
        return new RuneSectionGroupImpl(node);
      }
      throw new AssertionError("Unknown element type: " + type);
    }
  }
}

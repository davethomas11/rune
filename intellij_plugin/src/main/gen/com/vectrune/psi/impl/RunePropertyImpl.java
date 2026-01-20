// This is a generated file. Not intended for manual editing.
package com.vectrune.psi.impl;

import java.util.List;
import org.jetbrains.annotations.*;
import com.intellij.lang.ASTNode;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiElementVisitor;
import com.intellij.psi.util.PsiTreeUtil;
import static com.vectrune.RuneTypes.*;
import com.intellij.extapi.psi.ASTWrapperPsiElement;
import com.vectrune.psi.*;

public class RunePropertyImpl extends ASTWrapperPsiElement implements RuneProperty {

  public RunePropertyImpl(@NotNull ASTNode node) {
    super(node);
  }

  public void accept(@NotNull RuneVisitor visitor) {
    visitor.visitProperty(this);
  }

  @Override
  public void accept(@NotNull PsiElementVisitor visitor) {
    if (visitor instanceof RuneVisitor) accept((RuneVisitor)visitor);
    else super.accept(visitor);
  }

  @Override
  @NotNull
  public PsiElement getKey() {
    return findNotNullChildByType(KEY);
  }

  @Override
  @NotNull
  public PsiElement getValue() {
    return findNotNullChildByType(VALUE);
  }

}

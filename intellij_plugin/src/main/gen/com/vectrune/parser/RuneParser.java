// This is a generated file. Not intended for manual editing.
package com.vectrune.parser;

import com.intellij.lang.PsiBuilder;
import com.intellij.lang.PsiBuilder.Marker;
import static com.vectrune.RuneTypes.*;
import static com.intellij.lang.parser.GeneratedParserUtilBase.*;
import com.intellij.psi.tree.IElementType;
import com.intellij.lang.ASTNode;
import com.intellij.psi.tree.TokenSet;
import com.intellij.lang.PsiParser;
import com.intellij.lang.LightPsiParser;

@SuppressWarnings({"SimplifiableIfStatement", "UnusedAssignment"})
public class RuneParser implements PsiParser, LightPsiParser {

  public ASTNode parse(IElementType root_, PsiBuilder builder_) {
    parseLight(root_, builder_);
    return builder_.getTreeBuilt();
  }

  public void parseLight(IElementType root_, PsiBuilder builder_) {
    boolean result_;
    builder_ = adapt_builder_(root_, builder_, this, null);
    Marker marker_ = enter_section_(builder_, 0, _COLLAPSE_, null);
    result_ = parse_root_(root_, builder_);
    exit_section_(builder_, 0, marker_, root_, result_, true, TRUE_CONDITION);
  }

  protected boolean parse_root_(IElementType root_, PsiBuilder builder_) {
    return parse_root_(root_, builder_, 0);
  }

  static boolean parse_root_(IElementType root_, PsiBuilder builder_, int level_) {
    return runeFile(builder_, level_ + 1);
  }

  /* ********************************************************** */
  // section_group | property | COMMENT
  static boolean item_(PsiBuilder builder_, int level_) {
    if (!recursion_guard_(builder_, level_, "item_")) return false;
    boolean result_;
    result_ = section_group(builder_, level_ + 1);
    if (!result_) result_ = property(builder_, level_ + 1);
    if (!result_) result_ = consumeToken(builder_, COMMENT);
    return result_;
  }

  /* ********************************************************** */
  // KEY EQUALS VALUE
  public static boolean property(PsiBuilder builder_, int level_) {
    if (!recursion_guard_(builder_, level_, "property")) return false;
    if (!nextTokenIs(builder_, KEY)) return false;
    boolean result_;
    Marker marker_ = enter_section_(builder_);
    result_ = consumeTokens(builder_, 0, KEY, EQUALS, VALUE);
    exit_section_(builder_, marker_, PROPERTY, result_);
    return result_;
  }

  /* ********************************************************** */
  // item_ *
  static boolean runeFile(PsiBuilder builder_, int level_) {
    if (!recursion_guard_(builder_, level_, "runeFile")) return false;
    while (true) {
      int pos_ = current_position_(builder_);
      if (!item_(builder_, level_ + 1)) break;
      if (!empty_element_parsed_guard_(builder_, "runeFile", pos_)) break;
    }
    return true;
  }

  /* ********************************************************** */
  // SECTION property*
  public static boolean section_group(PsiBuilder builder_, int level_) {
    if (!recursion_guard_(builder_, level_, "section_group")) return false;
    if (!nextTokenIs(builder_, SECTION)) return false;
    boolean result_;
    Marker marker_ = enter_section_(builder_);
    result_ = consumeToken(builder_, SECTION);
    result_ = result_ && section_group_1(builder_, level_ + 1);
    exit_section_(builder_, marker_, SECTION_GROUP, result_);
    return result_;
  }

  // property*
  private static boolean section_group_1(PsiBuilder builder_, int level_) {
    if (!recursion_guard_(builder_, level_, "section_group_1")) return false;
    while (true) {
      int pos_ = current_position_(builder_);
      if (!property(builder_, level_ + 1)) break;
      if (!empty_element_parsed_guard_(builder_, "section_group_1", pos_)) break;
    }
    return true;
  }

}

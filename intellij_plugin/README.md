# Rune Language Support for IntelliJ

This IntelliJ IDEA plugin provides syntax highlighting and language support for **Rune** files (`.rune`).

## 🚀 Getting Started

### 1. Prerequisites

* **IntelliJ IDEA** (Community or Ultimate)
* **Java 17 or 21** (matching your project configuration)
* **JFlex JAR**: Ensure `libs/jflex-full-1.9.1.jar` is present in the project root.

### 2. Generating the Lexer

Before running the plugin for the first time or after modifying `Rune.flex`, you must generate the Java lexer:

```bash
./gradlew generateLexer

```

This will generate the `_RuneLexer.java` file in `src/main/gen/com/vectrune`.

### 3. Running the Plugin

To test the plugin in a sandbox instance of IntelliJ:

1. Open the **Gradle** side panel in IntelliJ.
2. Run **Tasks > intellij > runIde**.
3. In the new IDE window that opens, create or open a `.rune` file to see syntax highlighting in action.

---

## 🛠 Project Structure

* `src/main/lexer/Rune.flex`: The JFlex specification for the Rune language.
* `src/main/gen/`: Generated lexer files (ignored by Git, but used for compilation).
* `src/main/kotlin/`:
* `RuneLanguage.kt`: Defines the language instance.
* `RuneFileType.kt`: Associates the `.rune` extension.
* `RuneLexerAdapter.kt`: Wraps the JFlex lexer for IntelliJ.
* `RuneSyntaxHighlighter.kt`: Maps tokens (Comments, Keys, Sections) to colors.



---

## 📝 Syntax Rules

Currently supported patterns:
| Element | Syntax |
| :--- | :--- |
| **Comments** | `# This is a comment` |
| **Sections** | `@SectionName` |
| **Keys** | `variable_name` |
| **Assignments** | `=` |

---

## 📦 Building for Distribution

To create a plugin package that you can share or install in your primary IDE:

```bash
./gradlew buildPlugin

```

The resulting ZIP file will be located in: `build/distributions/rune-intellij-plugin-0.1.0.zip`.

### Installation

1. Go to **Settings/Preferences > Plugins**.
2. Click the ⚙️ icon and select **Install Plugin from Disk...**.
3. Select the ZIP file generated above.

---

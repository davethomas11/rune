plugins {
    id("org.jetbrains.intellij") version "1.17.3"
    id("org.jetbrains.grammarkit") version "2022.3.2"
    kotlin("jvm") version "1.9.23"
}

group = "com.vectrune"
version = "0.1.0"

repositories {
    mavenCentral()
}

intellij {
    version.set("2023.3")
    type.set("IC") // IntelliJ Community Edition
    plugins.set(listOf())
}

grammarKit {
    jflexRelease.set("1.9.1")
    grammarKitRelease.set("2022.3.2")
}

tasks {
    patchPluginXml {
        sinceBuild.set("233")
        untilBuild.set("241.*")
    }

    generateParser {
        sourceFile.set(file("src/main/lexer/Rune.bnf"))
        targetRoot.set("src/main/gen")
        pathToParser.set("com/vectrune/parser/RuneParser.java")
        pathToPsiRoot.set("com/vectrune/psi")
        purgeOldFiles.set(true)
    }

    generateLexer {
        sourceFile.set(file("src/main/lexer/Rune.flex"))
        targetDir.set("src/main/gen/com/vectrune")
        targetClass.set("_RuneLexer")
        purgeOldFiles.set(true)
        skeleton.set(file("src/main/lexer/idea-flex.skeleton"))
    }

    compileJava {
        dependsOn(generateLexer, generateParser)
    }

    compileKotlin {
        dependsOn(generateLexer, generateParser)
    }
}

dependencies {
    implementation(kotlin("stdlib"))
}

sourceSets {
    main {
        java.srcDirs("src/main/gen")
        kotlin.srcDirs("src/main/kotlin")
        resources.srcDirs("src/main/resources")
    }
}


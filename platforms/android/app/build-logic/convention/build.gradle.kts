import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
  `kotlin-dsl`
}

java {
  sourceCompatibility = JavaVersion.VERSION_17
  targetCompatibility = JavaVersion.VERSION_17
}

kotlin {
  compilerOptions {
    jvmTarget.set(JvmTarget.JVM_17)
  }
}

dependencies {
  implementation(projects.common)

  compileOnly(libs.android.gradle.plugin)
  compileOnly(libs.kotlin.gradle.plugin)
  compileOnly(libs.kover.gradle.plugin)
  compileOnly(libs.test.logger.gradle.plugin)
  compileOnly(libs.spotless.gradle.plugin)
}

gradlePlugin {
  plugins {
    register("androidApplication") {
      id = "tiebax.android.application"
      implementationClass = "AndroidApplicationConventionPlugin"
    }
    register("androidApplicationCompose") {
      id = "tiebax.android.application.compose"
      implementationClass = "AndroidApplicationComposeConventionPlugin"
    }
    register("androidApplicationKover") {
      id = "tiebax.android.application.kover"
      implementationClass = "AndroidApplicationKoverConventionPlugin"
    }
    register("androidLibrary") {
      id = "tiebax.android.library"
      implementationClass = "AndroidLibraryConventionPlugin"
    }
    register("androidLibraryCompose") {
      id = "tiebax.android.library.compose"
      implementationClass = "AndroidLibraryComposeConventionPlugin"
    }
    register("androidLibraryKover") {
      id = "tiebax.android.library.kover"
      implementationClass = "AndroidLibraryKoverConventionPlugin"
    }
    register("tiebxKover") {
      id = "tiebax.kover"
      implementationClass = "TiebaxKoverConventionPlugin"
    }
    register("tiebxSpotless") {
      id = "tiebax.spotless"
      implementationClass = "TiebaxSpotlessConventionPlugin"
    }
  }
}

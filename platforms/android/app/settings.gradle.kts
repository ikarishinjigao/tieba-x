pluginManagement {
  includeBuild("build-logic")
  repositories {
    google()
    mavenCentral()
    gradlePluginPortal()
    maven(url = "https://plugins.gradle.org/m2/")
  }
}

@Suppress("UnstableApiUsage")
dependencyResolutionManagement {
  repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
  repositories {
    google()
    mavenCentral()
    maven(url = "https://jitpack.io")
  }
}

rootProject.name = "tieba-x"

enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

include(":app")

include(":core:ffi")
include(":core:common")
include(":core:crypto")
include(":core:designsystem")
include(":core:network")

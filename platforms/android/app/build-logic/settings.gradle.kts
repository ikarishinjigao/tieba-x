@Suppress("UnstableApiUsage")
dependencyResolutionManagement {
  repositories {
    google()
    mavenCentral()
    maven(url = "https://plugins.gradle.org/m2/")
  }
  versionCatalogs {
    create("libs") {
      from(files("../gradle/libs.versions.toml"))
    }
  }
}

rootProject.name = "build-logic"

enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

include(":common")
include(":convention")

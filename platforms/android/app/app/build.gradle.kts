import cc.ikarishinjigao.tiebax.TiebaxBuildType
import cc.ikarishinjigao.tiebax.Version
import java.util.Properties

plugins {
  alias(libs.plugins.tiebax.android.application)
  alias(libs.plugins.tiebax.android.application.compose)
  alias(libs.plugins.tiebax.android.application.kover)
}

android {
  val packageName = "cc.ikarishinjigao.tiebax"

  namespace = packageName

  defaultConfig {
    applicationId = packageName
    versionCode = Version.App.VERSION_CODE
    versionName = Version.App.VERSION_NAME

    vectorDrawables {
      useSupportLibrary = true
    }
  }

  signingConfigs {
    val credentialProperties =
      Properties().apply {
        load(project.rootProject.file("credentials/credential.properties").inputStream())
      }
    getByName("debug") {
      storeFile = file("$rootDir/credentials/debug.keystore")
    }
    create("release") {
      storeFile = file("$rootDir/credentials/release.keystore")
      storePassword = credentialProperties.getProperty("RELEASE_STORE_PASSWORD")
      keyAlias = credentialProperties.getProperty("RELEASE_KEY_ALIAS")
      keyPassword = credentialProperties.getProperty("RELEASE_KEY_PASSWORD")
    }
  }

  buildTypes {
    debug {
      applicationIdSuffix = TiebaxBuildType.DEBUG.applicationIdSuffix
      signingConfig = signingConfigs.getByName("debug")
    }
    release {
      applicationIdSuffix = TiebaxBuildType.RELEASE.applicationIdSuffix
      isMinifyEnabled = true
      isShrinkResources = true
      signingConfig = signingConfigs.getByName("release")
      proguardFile(getDefaultProguardFile("proguard-android-optimize.txt"))
      proguardFile(file("proguard-rules.pro"))
    }
  }

  packaging {
    resources {
      excludes.add("/META-INF/{AL2.0,LGPL2.1,versions/**}")
    }
  }

  @Suppress("UnstableApiUsage")
  testOptions {
    unitTests {
      isIncludeAndroidResources = true
    }
  }
}

dependencies {
  implementation(projects.core.common)
  implementation(projects.core.crypto)
  implementation(projects.core.designsystem)
  implementation(projects.core.network)

  implementation(libs.androidx.activity.compose)
  implementation(libs.androidx.compose.material3)
  implementation(libs.androidx.navigation.compose)

  implementation(platform(libs.koin.bom))
  implementation(libs.koin.core)
  implementation(libs.koin.androidx.startup)
  implementation(libs.koin.androidx.compose)
  implementation(libs.koin.androidx.compose.navigation)

  implementation(libs.timber)
}

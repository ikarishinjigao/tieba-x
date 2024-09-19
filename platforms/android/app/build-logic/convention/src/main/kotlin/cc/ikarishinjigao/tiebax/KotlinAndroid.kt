package cc.ikarishinjigao.tiebax

import com.android.build.api.dsl.CommonExtension
import org.gradle.api.JavaVersion
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.dependencies
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.dsl.KotlinAndroidProjectExtension
import org.jetbrains.kotlin.gradle.dsl.KotlinTopLevelExtension

internal fun Project.configureKotlinAndroid(
  commonExtension: CommonExtension<*, *, *, *, *, *>,
) {
  commonExtension.apply {
    compileSdk = Version.Android.COMPILE_SDK

    defaultConfig {
      minSdk = Version.Android.MIN_SDK
    }

    buildFeatures {
      buildConfig = true
    }

    compileOptions {
      sourceCompatibility = JavaVersion.VERSION_11
      targetCompatibility = JavaVersion.VERSION_11
      isCoreLibraryDesugaringEnabled = true
    }
  }

  dependencies {
    add("coreLibraryDesugaring", libs.findLibrary("android-desugar-jdk-libs").get())
  }

  configureKotlin<KotlinAndroidProjectExtension>()
}

private inline fun <reified T : KotlinTopLevelExtension> Project.configureKotlin() = configure<T> {
  when (this) {
    is KotlinAndroidProjectExtension -> compilerOptions
    else -> TODO("unsupported project extension $this ${T::class}")
  }.apply {
    jvmTarget.set(JvmTarget.JVM_11)
  }
}

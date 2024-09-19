package cc.ikarishinjigao.tiebax

import com.android.build.api.dsl.ApplicationExtension
import com.android.build.api.dsl.ApplicationProductFlavor
import com.android.build.api.dsl.CommonExtension
import org.gradle.api.Project
import java.util.Properties

enum class FlavorDimension {
  DISTRIBUTION_TARGET,
}

enum class TiebaxFlavor(
  val dimension: FlavorDimension,
  val applicationIdSuffix: String? = null,
) {
  PRODUCTION(dimension = FlavorDimension.DISTRIBUTION_TARGET),
}

internal fun Project.configureFlavors(
  commonExtension: CommonExtension<*, *, *, *, *, *>,
) {
  val credentialProperties = Properties().apply {
    load(project.rootProject.file("credentials/credential.properties").inputStream())
  }

  commonExtension.apply {
    signingConfigs {
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

    flavorDimensions += FlavorDimension.DISTRIBUTION_TARGET.name

    productFlavors {
      TiebaxFlavor.values().forEach { flavor ->
        create(flavor.name.lowercase()) {
          dimension = flavor.dimension.name
          if (this@apply is ApplicationExtension && this is ApplicationProductFlavor) {
            flavor.applicationIdSuffix?.let {
              applicationIdSuffix = it
            }
          }
        }
      }
    }
  }
}

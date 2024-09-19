package cc.ikarishinjigao.tiebax

import cc.ikarishinjigao.tiebax.KoverConfig.AGGREGATED_VARIANT_NAME
import cc.ikarishinjigao.tiebax.KoverConfig.REPORT_VARIANT_NAME
import cc.ikarishinjigao.tiebax.KoverConfig.excludeClasses
import cc.ikarishinjigao.tiebax.KoverConfig.excludePackages
import com.adarshr.gradle.testlogger.TestLoggerExtension
import com.adarshr.gradle.testlogger.theme.ThemeType
import kotlinx.kover.gradle.plugin.dsl.KoverProjectExtension
import org.gradle.api.Project
import org.gradle.api.tasks.testing.Test
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.withType

internal object KoverConfig {
  const val AGGREGATED_VARIANT_NAME = "aggregated"
  const val REPORT_VARIANT_NAME = "productionDebug"

  val excludePackages = listOf(
    "dagger",
    "hilt_aggregated_deps",
    "*.protobuf",
    "*.protobuf.*",
  )

  val excludeClasses = listOf(
    "*_MembersInjector*",
    "*_Provide*Factory*",
    "*ComposableSingletons\$*",
  )
}

internal fun Project.configureKover() {
  configure<KoverProjectExtension> {
    currentProject {
      createVariant(AGGREGATED_VARIANT_NAME) {
        add(REPORT_VARIANT_NAME)
      }
    }
    reports {
      filters {
        excludes {
          androidGeneratedClasses()
          packages(excludePackages)
          classes(excludeClasses)
        }
      }
    }
  }
  configureTestTasks()
}

internal fun Project.configureKoverRoot() {
  configure<KoverProjectExtension> {
    merge {
      subprojects()
      createVariant(AGGREGATED_VARIANT_NAME) {}
    }
    reports {
      filters {
        excludes {
          androidGeneratedClasses()
          packages(excludePackages)
          classes(excludeClasses)
        }
      }
    }
  }
}

private fun Project.configureTestTasks() {
  tasks.withType<Test>().configureEach {
    configure<TestLoggerExtension> {
      showSimpleNames = true
      theme = ThemeType.MOCHA_PARALLEL
    }
  }
}

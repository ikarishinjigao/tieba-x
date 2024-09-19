import cc.ikarishinjigao.tiebax.libs
import com.diffplug.gradle.spotless.SpotlessExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure

class TiebaxSpotlessConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      allprojects {
        afterEvaluate {
          with(pluginManager) {
            apply(libs.findPlugin("spotless").get().get().pluginId)
          }

          extensions.configure<SpotlessExtension> {
            kotlin {
              target("**/*.kt")
              targetExclude("**/build/**/*.kt")
              ktlint(libs.findVersion("ktlint").get().toString())
                .editorConfigOverride(
                  mapOf(
                    "ktlint_function_naming_ignore_when_annotated_with" to "Composable"
                  )
                )
            }
            kotlinGradle {
              target("**/*.gradle.kts")
              ktlint()
            }
          }
        }
      }
    }
  }
}

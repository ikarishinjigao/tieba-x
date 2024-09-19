import cc.ikarishinjigao.tiebax.configureKover
import cc.ikarishinjigao.tiebax.libs
import com.android.build.api.dsl.ApplicationExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.getByType

class AndroidApplicationKoverConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      with(pluginManager) {
        apply(libs.findPlugin("android-application").get().get().pluginId)
        apply(libs.findPlugin("kover").get().get().pluginId)
        apply(libs.findPlugin("test-logger").get().get().pluginId)
      }

      extensions.getByType<ApplicationExtension>().buildTypes.configureEach {
        enableAndroidTestCoverage = true
        enableUnitTestCoverage = true
      }

      configureKover()
    }
  }
}

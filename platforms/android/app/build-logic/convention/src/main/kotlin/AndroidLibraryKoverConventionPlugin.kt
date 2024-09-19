import cc.ikarishinjigao.tiebax.configureKover
import cc.ikarishinjigao.tiebax.libs
import com.android.build.api.dsl.LibraryExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.getByType

class AndroidLibraryKoverConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      with(pluginManager) {
        apply(libs.findPlugin("android-library").get().get().pluginId)
        apply(libs.findPlugin("kover").get().get().pluginId)
        apply(libs.findPlugin("test-logger").get().get().pluginId)
      }

      extensions.getByType<LibraryExtension>().buildTypes.configureEach {
        enableAndroidTestCoverage = true
        enableUnitTestCoverage = true
      }

      configureKover()
    }
  }
}

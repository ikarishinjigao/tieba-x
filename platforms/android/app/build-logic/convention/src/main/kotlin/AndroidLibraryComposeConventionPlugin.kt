import cc.ikarishinjigao.tiebax.configureAndroidCompose
import cc.ikarishinjigao.tiebax.libs
import com.android.build.api.dsl.LibraryExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure

class AndroidLibraryComposeConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      with(pluginManager) {
        apply(libs.findPlugin("android-library").get().get().pluginId)
        apply(libs.findPlugin("kotlin-compose").get().get().pluginId)
      }

      extensions.configure<LibraryExtension> {
        configureAndroidCompose(this)
      }
    }
  }
}

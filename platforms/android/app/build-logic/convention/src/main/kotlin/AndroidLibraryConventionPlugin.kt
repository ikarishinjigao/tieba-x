import cc.ikarishinjigao.tiebax.configureGradleManagedDevices
import cc.ikarishinjigao.tiebax.configureKotlinAndroid
import cc.ikarishinjigao.tiebax.disableUnnecessaryAndroidTests
import cc.ikarishinjigao.tiebax.libs
import com.android.build.api.dsl.LibraryExtension
import com.android.build.api.variant.LibraryAndroidComponentsExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.dependencies

class AndroidLibraryConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      with(pluginManager) {
        apply(libs.findPlugin("android-library").get().get().pluginId)
        apply(libs.findPlugin("kotlin-android").get().get().pluginId)
      }

      extensions.configure<LibraryExtension> {
        configureKotlinAndroid(this)
        configureGradleManagedDevices(this)
      }

      extensions.configure<LibraryAndroidComponentsExtension> {
        disableUnnecessaryAndroidTests(target)
      }

      dependencies {
        add("testImplementation", libs.findLibrary("kotlin.test").get())
      }
    }
  }
}

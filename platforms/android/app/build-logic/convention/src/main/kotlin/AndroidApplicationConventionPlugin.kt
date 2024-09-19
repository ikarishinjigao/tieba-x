import cc.ikarishinjigao.tiebax.Version
import cc.ikarishinjigao.tiebax.configureFlavors
import cc.ikarishinjigao.tiebax.configureKotlinAndroid
import cc.ikarishinjigao.tiebax.libs
import com.android.build.api.dsl.ApplicationExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure

class AndroidApplicationConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      with(pluginManager) {
        apply(libs.findPlugin("android-application").get().get().pluginId)
        apply(libs.findPlugin("kotlin-android").get().get().pluginId)
      }

      extensions.configure<ApplicationExtension> {
        configureKotlinAndroid(this)
        configureFlavors(this)
        defaultConfig.targetSdk = Version.Android.TARGET_SDK
      }
    }
  }
}

import cc.ikarishinjigao.tiebax.configureKoverRoot
import cc.ikarishinjigao.tiebax.libs
import org.gradle.api.Plugin
import org.gradle.api.Project

class TiebaxKoverConventionPlugin : Plugin<Project> {
  override fun apply(target: Project) {
    with(target) {
      with(pluginManager) {
        apply(libs.findPlugin("kover").get().get().pluginId)
      }

      configureKoverRoot()
    }
  }
}

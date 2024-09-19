plugins {
  id(libs.plugins.tiebax.kover.get().pluginId)
  id(libs.plugins.tiebax.spotless.get().pluginId)

  alias(libs.plugins.android.application) apply false
  alias(libs.plugins.android.library) apply false
  alias(libs.plugins.kotlin.android) apply false
  alias(libs.plugins.kotlin.serialization) apply false
  alias(libs.plugins.kotlin.compose) apply false
  alias(libs.plugins.kover) apply false
  alias(libs.plugins.test.logger) apply false
  alias(libs.plugins.spotless) apply true
}

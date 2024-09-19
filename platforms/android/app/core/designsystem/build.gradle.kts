plugins {
  alias(libs.plugins.tiebax.android.library)
  alias(libs.plugins.tiebax.android.library.compose)
  alias(libs.plugins.tiebax.android.library.kover)
}

android {
  namespace = "cc.ikarishinjigao.tiebax.core.designsystem"
}

dependencies {
  api(libs.androidx.compose.runtime)
  api(libs.androidx.compose.foundation)
  api(libs.androidx.compose.foundation.layout)
  api(libs.androidx.compose.material3)
  api(libs.androidx.compose.ui.tooling.preview)
  api(libs.androidx.compose.ui.util)

  debugApi(libs.androidx.compose.ui.tooling)

  implementation(platform(libs.kotlinx.coroutines.bom))
  implementation(libs.kotlinx.coroutines.core)
  implementation(libs.androidx.core.ktx)
}

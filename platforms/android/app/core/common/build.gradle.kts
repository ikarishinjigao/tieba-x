plugins {
  alias(libs.plugins.tiebax.android.library)
  alias(libs.plugins.tiebax.android.library.kover)
}

android {
  namespace = "cc.ikarishinjigao.tiebax.core.common"
}

dependencies {
  implementation(platform(libs.kotlinx.coroutines.bom))
  implementation(libs.kotlinx.coroutines.core)
}

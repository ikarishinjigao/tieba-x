plugins {
  alias(libs.plugins.tiebax.android.library)
}

android {
  namespace = "cc.ikarishinjigao.tiebax.core.network"
}

dependencies {
  implementation(projects.core.ffi)

  implementation(libs.androidx.annotaion)
  implementation(platform(libs.kotlinx.coroutines.bom))
  implementation(libs.kotlinx.coroutines.core)
  implementation(platform(libs.koin.bom))
  implementation(libs.koin.core)
}

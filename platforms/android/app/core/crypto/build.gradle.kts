plugins {
  alias(libs.plugins.tiebax.android.library)
}

android {
  namespace = "cc.ikarishinjigao.tiebax.core.crypto"
}

dependencies {
  implementation(projects.core.ffi)

  implementation(libs.androidx.annotaion)
  implementation(platform(libs.koin.bom))
  implementation(libs.koin.core)
}

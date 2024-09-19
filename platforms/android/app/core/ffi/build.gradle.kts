plugins {
  alias(libs.plugins.tiebax.android.library)
}

android {
  namespace = "cc.ikarishinjigao.tiebax.core.ffi"
}

dependencies {
  api(libs.jna) {
    artifact {
      type = "aar"
    }
  }
}

package cc.ikarishinjigao.tiebax.core.common.system

import android.annotation.SuppressLint

@SuppressLint("PrivateApi")
object SystemProperties {
  private val systemProperties by lazy { Class.forName("android.os.SystemProperties") }

  enum class Manufacturer {
    XIAOMI,
    OTHER,
  }

  private fun get(key: String, defaultValue: String = ""): String =
    runCatching {
      systemProperties
        .getMethod("get", String::class.java, String::class.java)
        .invoke(null, key, defaultValue) as String
    }.getOrDefault(defaultValue)

  fun currentManufacturer(): Manufacturer {
    val isMiOs = get("ro.mi.os.version.code").isNotBlank()
    val isMiUi = get("ro.miui.ui.version.code").isNotBlank()
    return if (isMiOs || isMiUi) {
      Manufacturer.XIAOMI
    } else {
      Manufacturer.OTHER
    }
  }
}

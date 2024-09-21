package cc.ikarishinjigao.tiebax.extension

import android.content.res.Configuration
import android.content.res.Resources
import android.graphics.Color
import android.view.Window
import android.view.WindowManager
import androidx.activity.ComponentActivity
import androidx.activity.SystemBarStyle
import androidx.activity.enableEdgeToEdge
import cc.ikarishinjigao.tiebax.core.common.system.SystemProperties

private val lightScrim = Color.argb(0xe6, 0xFF, 0xFF, 0xFF)
private val darkScrim = Color.argb(0x80, 0x1b, 0x1b, 0x1b)

fun ComponentActivity.enableEdgeToEdge(
  detectDarkMode: (Resources) -> Boolean = {
    val mode = it.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK
    mode == Configuration.UI_MODE_NIGHT_YES
  },
  statusBarStyle: SystemBarStyle = SystemBarStyle.auto(
    lightScrim = Color.TRANSPARENT,
    darkScrim = Color.TRANSPARENT,
    detectDarkMode = detectDarkMode,
  ),
  navigationBarStyle: SystemBarStyle = SystemBarStyle.auto(
    lightScrim = lightScrim,
    darkScrim = darkScrim,
    detectDarkMode = detectDarkMode,
  ),
) {
  configureWindowByManufacturer(window)
  enableEdgeToEdge(
    statusBarStyle = statusBarStyle,
    navigationBarStyle = navigationBarStyle,
  )
}

@Suppress("DEPRECATION")
private fun configureWindowByManufacturer(window: Window) {
  when (SystemProperties.currentManufacturer()) {
    SystemProperties.Manufacturer.XIAOMI -> {
      window.addFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS)
      window.addFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_NAVIGATION)
    }

    SystemProperties.Manufacturer.OTHER -> {}
  }
}

package cc.ikarishinjigao.tiebax

import android.graphics.Color.TRANSPARENT
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.SystemBarStyle
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.DisposableEffect
import cc.ikarishinjigao.tiebax.core.designsystem.theme.TiebaxTheme
import cc.ikarishinjigao.tiebax.ui.TiebaxApp
import org.koin.compose.KoinContext

class TiebaxActivity : ComponentActivity() {
  override fun onCreate(
    savedInstanceState: Bundle?,
  ) {
    super.onCreate(savedInstanceState)
    enableEdgeToEdge()
    setContent {
      DisposableEffect(true) {
        enableEdgeToEdge(
          statusBarStyle = SystemBarStyle.auto(
            TRANSPARENT,
            TRANSPARENT,
          ),
          navigationBarStyle = SystemBarStyle.auto(
            lightScrim,
            darkScrim,
          ),
        )
        onDispose {}
      }
      KoinContext {
        TiebaxTheme {
          TiebaxApp()
        }
      }
    }
  }
}

private val lightScrim = android.graphics.Color.argb(0xe6, 0xFF, 0xFF, 0xFF)
private val darkScrim = android.graphics.Color.argb(0x80, 0x1b, 0x1b, 0x1b)

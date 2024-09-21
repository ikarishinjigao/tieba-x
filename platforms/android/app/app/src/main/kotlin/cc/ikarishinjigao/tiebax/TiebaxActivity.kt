package cc.ikarishinjigao.tiebax

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import cc.ikarishinjigao.tiebax.core.designsystem.theme.TiebaxTheme
import cc.ikarishinjigao.tiebax.extension.enableEdgeToEdge
import cc.ikarishinjigao.tiebax.ui.TiebaxApp
import org.koin.compose.KoinContext

class TiebaxActivity : ComponentActivity() {
  override fun onCreate(
    savedInstanceState: Bundle?,
  ) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    setContent {
      KoinContext {
        TiebaxTheme {
          TiebaxApp()
        }
      }
    }
  }
}

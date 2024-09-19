package cc.ikarishinjigao.tiebax.core.designsystem.component

import android.graphics.Bitmap
import android.view.ViewGroup.LayoutParams
import android.webkit.CookieManager
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.FrameLayout
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.viewinterop.AndroidView

@Composable
fun WebView(
  modifier: Modifier = Modifier,
  state: WebViewState,
  webSettings: WebSettings.() -> Unit = {},
  shouldOverrideUrlLoading: (String) -> Boolean = { false },
  onPageFinished: (String) -> Unit = {},
) {
  BoxWithConstraints(modifier) {
    val width = if (constraints.hasFixedWidth) {
      LayoutParams.MATCH_PARENT
    } else {
      LayoutParams.WRAP_CONTENT
    }
    val height = if (constraints.hasFixedHeight) {
      LayoutParams.MATCH_PARENT
    } else {
      LayoutParams.WRAP_CONTENT
    }

    WebView(
      state = state,
      layoutParams = FrameLayout.LayoutParams(
        width,
        height,
      ),
      webSettings = webSettings,
      client = remember {
        InternalWebViewClient(
          shouldOverrideUrlLoading = shouldOverrideUrlLoading,
          onPageFinished = onPageFinished,
        )
      },
    )
  }
}

@Composable
internal fun WebView(
  modifier: Modifier = Modifier,
  state: WebViewState,
  layoutParams: FrameLayout.LayoutParams,
  webSettings: WebSettings.() -> Unit,
  client: InternalWebViewClient,
) {
  val isInPreview = LocalInspectionMode.current

  state.webView?.let {
    LaunchedEffect(it, state) {
      it.loadUrl(state.url)
    }
  }

  AndroidView(
    modifier = modifier,
    factory = { context ->
      WebView(context)
        .apply {
          this.layoutParams = layoutParams
          webViewClient = client
          if (isInPreview.not()) {
            webSettings(settings)
          }
        }
        .also { state.webView = it }
    },
  )
}

@Composable
fun rememberWebViewState(
  url: String,
): WebViewState = remember {
  WebViewState(url = url)
}

class WebViewState(
  url: String,
) {
  var url: String by mutableStateOf(url)
  internal var webView: WebView? by mutableStateOf(null)
}

internal class InternalWebViewClient(
  val shouldOverrideUrlLoading: (String) -> Boolean,
  val onPageFinished: (String) -> Unit,
) : WebViewClient() {
  override fun shouldOverrideUrlLoading(
    view: WebView?,
    request: WebResourceRequest?,
  ): Boolean {
    val url = request?.url?.toString() ?: return false
    println("override $url, redirect: ${request.isRedirect}")
    return shouldOverrideUrlLoading(url)
  }

  override fun onPageStarted(
    view: WebView?,
    url: String?,
    favicon: Bitmap?
  ) {
    super.onPageStarted(view, url, favicon)
    if (url.isNullOrEmpty()) return
    println("start loading: $url")
  }

  override fun onPageFinished(
    view: WebView?,
    url: String?,
  ) {
    super.onPageFinished(view, url)
    if (url.isNullOrEmpty()) return
    println("end loading: $url")
    onPageFinished(url)
  }
}

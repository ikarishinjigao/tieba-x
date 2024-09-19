package cc.ikarishinjigao.tiebax.core.common.extension

import java.net.URLEncoder
import java.nio.charset.StandardCharsets

fun String.urlEncoded(): String =
  URLEncoder.encode(this, StandardCharsets.UTF_8.toString())

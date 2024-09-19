package cc.ikarishinjigao.tiebax

enum class TiebaxBuildType(
  val applicationIdSuffix: String? = null,
) {
  DEBUG(applicationIdSuffix = ".debug"),
  RELEASE,
}

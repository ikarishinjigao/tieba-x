package cc.ikarishinjigao.tiebax.di

import cc.ikarishinjigao.tiebax.core.crypto.cryptoModule
import cc.ikarishinjigao.tiebax.core.network.networkModule
import cc.ikarishinjigao.tiebax.ui.TiebaxAppViewModel
import org.koin.core.module.dsl.viewModelOf
import org.koin.dsl.module

val appModule = module {
  includes(
    cryptoModule,
    networkModule,
  )
  viewModelOf(::TiebaxAppViewModel)
}

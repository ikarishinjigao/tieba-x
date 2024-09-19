package cc.ikarishinjigao.tiebax.core.network

import org.koin.dsl.module

val networkModule = module {
  single<ApiClientInterface> { ApiClient() }
}

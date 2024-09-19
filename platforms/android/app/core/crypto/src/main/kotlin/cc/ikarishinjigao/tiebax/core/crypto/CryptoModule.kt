package cc.ikarishinjigao.tiebax.core.crypto

import org.koin.dsl.module

val cryptoModule = module {
    single<CuidInterface> { Cuid() }
}

package cc.ikarishinjigao.tiebax

import android.app.Application
import cc.ikarishinjigao.tiebax.di.appModule
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.androix.startup.KoinStartup.onKoinStartup
import org.koin.core.logger.Level
import timber.log.Timber

@Suppress("OPT_IN_USAGE")
class TiebaxApplication : Application() {
  init {
    onKoinStartup {
      androidLogger(Level.DEBUG)
      androidContext(this@TiebaxApplication)
      modules(appModule)
    }
    if (BuildConfig.DEBUG) {
      Timber.plant(Timber.DebugTree())
    }
  }
}

package cc.ikarishinjigao.tiebax.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import cc.ikarishinjigao.tiebax.core.crypto.IdManagerInterface
import cc.ikarishinjigao.tiebax.core.network.ApiClientInterface
import cc.ikarishinjigao.tiebax.core.network.GetThreadsRequest
import cc.ikarishinjigao.tiebax.core.network.ThreadSortType
import kotlinx.coroutines.launch
import timber.log.Timber
import kotlin.random.Random
import kotlin.random.nextULong

class TiebaxAppViewModel(
  private val idManager: IdManagerInterface,
  private val apiClient: ApiClientInterface,
) : ViewModel() {
  fun testApiClient() = viewModelScope.launch {
    runCatching {
      apiClient.getThreads(
        GetThreadsRequest(
          forumName = "amd",
          pageNumber = 1,
          pageSize = 5,
          sortType = ThreadSortType.CREATE_TIME,
          isHighQualityThread = false,
        ),
      )
    }.onSuccess {
      Timber.d("$it")
    }.onFailure {
      Timber.e(it)
    }
  }

  fun testCuid() {
    val randomGenerator = Random(System.currentTimeMillis())
    val seed = randomGenerator.nextULong()
    val uuid = idManager.generateUuid()
    val androidId = idManager.generateAndroidId(seed)
    val cuid = idManager.generateCuid(androidId)
    val c3Aid = idManager.generateC3Aid(androidId, uuid)
    Timber.d("$androidId\n$cuid\n$c3Aid")
  }
}

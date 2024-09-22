package cc.ikarishinjigao.tiebax.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import cc.ikarishinjigao.tiebax.core.crypto.CuidInterface
import cc.ikarishinjigao.tiebax.core.network.ApiClientInterface
import cc.ikarishinjigao.tiebax.core.network.GetThreadsRequest
import cc.ikarishinjigao.tiebax.core.network.ThreadSortType
import kotlinx.coroutines.launch
import timber.log.Timber

class TiebaxAppViewModel(
  private val apiClient: ApiClientInterface,
  private val cuid: CuidInterface,
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
    val result = cuid.generateCuid("52ee55117d525049")
    Timber.d(result)
  }
}

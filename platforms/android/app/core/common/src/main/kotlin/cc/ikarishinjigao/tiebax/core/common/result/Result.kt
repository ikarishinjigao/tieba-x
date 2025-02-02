package cc.ikarishinjigao.tiebax.core.common.result

import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.onStart

sealed interface Result<out T> {
  data object Loading : Result<Nothing>
  data class Success<T>(val data: T) : Result<T>
  data class Error(val exception: Throwable? = null) : Result<Nothing>
}

fun <T> Flow<T>.asResult(): Flow<Result<T>> =
  this
    .map<T, Result<T>> { Result.Success(it) }
    .onStart { emit(Result.Loading) }
    .catch { emit(Result.Error(it)) }

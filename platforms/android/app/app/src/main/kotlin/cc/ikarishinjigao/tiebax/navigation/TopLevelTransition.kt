package cc.ikarishinjigao.tiebax.navigation

import androidx.compose.animation.EnterTransition
import androidx.compose.animation.ExitTransition
import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.navigation.NavBackStackEntry
import cc.ikarishinjigao.tiebax.ui.TiebaxAppState

private const val TRANSITION_DURATION = 350
private const val TRANSITION_OFFSET = 0.064f

fun createEnterTransition(
  initialState: NavBackStackEntry,
  targetState: NavBackStackEntry,
  appState: TiebaxAppState,
): EnterTransition {
  val forward = isForwardNavigation(initialState, targetState, appState)
  val offset = if (forward) TRANSITION_OFFSET else -TRANSITION_OFFSET
  val easing = CubicBezierEasing(0.22f, 1f, 0.36f, 1f)
  return slideInHorizontally(
    animationSpec = tween(
      durationMillis = TRANSITION_DURATION / 2,
      easing = easing,
      delayMillis = TRANSITION_DURATION / 2,
    ),
  ) { (it * offset).toInt() } + fadeIn(
    animationSpec = tween(
      durationMillis = TRANSITION_DURATION / 2,
      easing = easing,
      delayMillis = TRANSITION_DURATION / 2,
    ),
  )
}

fun createExitTransition(
  initialState: NavBackStackEntry,
  targetState: NavBackStackEntry,
  appState: TiebaxAppState,
): ExitTransition {
  val forward = isForwardNavigation(initialState, targetState, appState)
  val offset = if (forward) -TRANSITION_OFFSET else TRANSITION_OFFSET
  val easing = CubicBezierEasing(0.64f, 0f, 0.78f, 0f)
  return slideOutHorizontally(
    animationSpec = tween(
      durationMillis = TRANSITION_DURATION / 2,
      easing = easing,
    ),
  ) { (it * offset).toInt() } + fadeOut(
    animationSpec = tween(
      durationMillis = TRANSITION_DURATION / 2,
      easing = easing,
    ),
  )
}

private fun isForwardNavigation(
  initialState: NavBackStackEntry,
  targetState: NavBackStackEntry,
  appState: TiebaxAppState,
): Boolean {
  val initialDestination = appState.findTopLevelDestination(
    initialState.destination,
  ) ?: return false
  val targetDestination = appState.findTopLevelDestination(
    targetState.destination,
  ) ?: return false
  return initialDestination.order < targetDestination.order
}

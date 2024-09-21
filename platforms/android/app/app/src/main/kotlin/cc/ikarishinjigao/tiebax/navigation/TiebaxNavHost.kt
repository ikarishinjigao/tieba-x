package cc.ikarishinjigao.tiebax.navigation

import androidx.compose.animation.AnimatedContentTransitionScope
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavBackStackEntry
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import cc.ikarishinjigao.tiebax.ui.Test
import cc.ikarishinjigao.tiebax.ui.TiebaxAppState
import org.koin.androidx.compose.koinViewModel

@Composable
fun TiebaxNavHost(
  appState: TiebaxAppState,
  modifier: Modifier = Modifier,
) {
  val navController = appState.navController
  val transitionDuration = 350
  NavHost(
    modifier = modifier,
    navController = navController,
    startDestination = appState.topLevelDestinations.first().name,
    enterTransition = {
      val forward = isForwardNavigation(initialState, targetState, appState)
      slideIntoContainer(
        towards = if (forward) {
          AnimatedContentTransitionScope.SlideDirection.Left
        } else {
          AnimatedContentTransitionScope.SlideDirection.Right
        },
        animationSpec = tween(transitionDuration),
      ) + fadeIn(animationSpec = tween(transitionDuration))
    },
    exitTransition = {
      val forward = isForwardNavigation(initialState, targetState, appState)
      slideOutOfContainer(
        towards = if (forward) {
          AnimatedContentTransitionScope.SlideDirection.Left
        } else {
          AnimatedContentTransitionScope.SlideDirection.Right
        },
        animationSpec = tween(transitionDuration),
      ) + fadeOut(animationSpec = tween(transitionDuration))
    },
  ) {
    composable(
      route = TopLevelDestination.SCREEN_1.name,
    ) {
      Test(viewModel = koinViewModel())
    }
    composable(
      route = TopLevelDestination.SCREEN_2.name,
    ) {
      Column(
        modifier = Modifier
          .fillMaxSize(),
      ) {
        Text("Screen ${it.destination.route}")
      }
    }
    composable(
      route = TopLevelDestination.SCREEN_3.name,
    ) {
      Column(
        modifier = Modifier
          .fillMaxSize(),
      ) {
        Text("Screen ${it.destination.route}")
      }
    }
    composable(
      route = TopLevelDestination.SCREEN_4.name,
    ) {
      Column(
        modifier = Modifier
          .fillMaxSize(),
      ) {
        Text("Screen ${it.destination.route}")
      }
    }
  }
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

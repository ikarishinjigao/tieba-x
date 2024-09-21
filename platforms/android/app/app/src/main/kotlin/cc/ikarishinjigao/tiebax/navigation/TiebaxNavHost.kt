package cc.ikarishinjigao.tiebax.navigation

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
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
  NavHost(
    modifier = modifier,
    navController = navController,
    startDestination = appState.topLevelDestinations.first().name,
    enterTransition = { createEnterTransition(initialState, targetState, appState) },
    exitTransition = { createExitTransition(initialState, targetState, appState) },
  ) {
    composable(TopLevelDestination.SCREEN_1.name) {
      Test(viewModel = koinViewModel())
    }
    composable(TopLevelDestination.SCREEN_2.name) {
      TestScreen(title = it.destination.route)
    }
    composable(TopLevelDestination.SCREEN_3.name) {
      TestScreen(title = it.destination.route)
    }
    composable(TopLevelDestination.SCREEN_4.name) {
      TestScreen(title = it.destination.route)
    }
  }
}

@Composable
fun TestScreen(
  title: String?,
) {
  Column(
    modifier = Modifier
      .fillMaxSize(),
    verticalArrangement = Arrangement.Center,
    horizontalAlignment = Alignment.CenterHorizontally,
  ) {
    Text("Screen $title")
  }
}

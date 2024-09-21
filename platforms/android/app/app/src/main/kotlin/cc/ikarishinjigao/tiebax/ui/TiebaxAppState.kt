package cc.ikarishinjigao.tiebax.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.Stable
import androidx.compose.runtime.remember
import androidx.navigation.NavDestination
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.NavHostController
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import cc.ikarishinjigao.tiebax.navigation.TopLevelDestination

@Composable
fun rememberTiebaxAppState(
  navController: NavHostController = rememberNavController(),
): TiebaxAppState {
  return remember(navController) {
    TiebaxAppState(
      navController = navController,
    )
  }
}

@Stable
class TiebaxAppState(
  val navController: NavHostController,
) {
  val currentTopLevelDestination: TopLevelDestination?
    @Composable get() =
      navController.currentBackStackEntryAsState().value?.destination?.let {
        findTopLevelDestination(destination = it)
      }

  val topLevelDestinations: List<TopLevelDestination> = TopLevelDestination.entries

  fun findTopLevelDestination(destination: NavDestination) =
    TopLevelDestination.entries.firstOrNull {
      destination.route == it.name
    }

  fun navigateToTopLevelDestination(topLevelDestination: TopLevelDestination) {
    navController.navigate(topLevelDestination.name) {
      popUpTo(navController.graph.findStartDestination().id) {
        saveState = true
      }
      launchSingleTop = true
      restoreState = true
    }
  }
}

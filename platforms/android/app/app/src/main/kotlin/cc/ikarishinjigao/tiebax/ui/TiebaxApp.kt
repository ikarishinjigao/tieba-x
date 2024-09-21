package cc.ikarishinjigao.tiebax.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Face
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import cc.ikarishinjigao.tiebax.navigation.TiebaxNavHost

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TiebaxApp() {
  val appState = rememberTiebaxAppState()
  Scaffold(
    topBar = {
      TopAppBar(title = { Text("Title") })
    },
    bottomBar = {
      NavigationBar {
        appState.topLevelDestinations.forEach { topLevelDestination ->
          NavigationBarItem(
            icon = { Icon(Icons.Default.Face, contentDescription = null) },
            label = { Text(topLevelDestination.title) },
            selected = appState.currentTopLevelDestination == topLevelDestination,
            onClick = { appState.navigateToTopLevelDestination(topLevelDestination) },
          )
        }
      }
    },
  ) { innerPadding ->
    TiebaxNavHost(
      modifier = Modifier
        .padding(innerPadding),
      appState = appState,
    )
  }
}

@Composable
fun Test(
  viewModel: TiebaxAppViewModel,
) {
  Column(
    modifier = Modifier
      .fillMaxSize(),
    verticalArrangement = Arrangement.Center,
    horizontalAlignment = Alignment.CenterHorizontally,
  ) {
    Button(
      onClick = {
        viewModel.testApiClient()
      },
    ) {
      Text("test api client")
    }
    Button(
      onClick = {
        viewModel.testCuid()
      },
    ) {
      Text("test cuid")
    }
  }
}

@Preview
@Composable
private fun TiebaAppPreview() {
  TiebaxApp()
}

package cc.ikarishinjigao.tiebax.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
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
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TiebaxApp(
  viewModel: TiebaAppViewModel = koinViewModel(),
) {
  var selectedItem by remember { mutableIntStateOf(0) }
  val items = listOf("Label", "Label", "Label", "Label")
  Scaffold(
    topBar = {
      TopAppBar(title = { Text("Title") })
    },
    bottomBar = {
      NavigationBar {
        items.forEachIndexed { index, item ->
          NavigationBarItem(
            icon = { Icon(Icons.Default.Face, contentDescription = item) },
            label = { Text(item) },
            selected = selectedItem == index,
            onClick = { selectedItem = index },
          )
        }
      }
    },
  ) { innerPadding ->
    Row(
      modifier = Modifier
        .padding(innerPadding),
    ) {
      Test(viewModel)
    }
  }
}

@Composable
fun Test(
  viewModel: TiebaAppViewModel,
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

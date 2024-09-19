package cc.ikarishinjigao.tiebax.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import org.koin.androidx.compose.koinViewModel

@Composable
fun TiebaxApp(
  viewModel: TiebaAppViewModel = koinViewModel(),
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

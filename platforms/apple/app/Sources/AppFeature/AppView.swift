import ComposableArchitecture
import SwiftUI

public struct AppView: View {
  let store: StoreOf<AppReducer>

  public init(
    store: StoreOf<AppReducer>
  ) {
    self.store = store
  }

  public var body: some View {
    VStack {
      Button("test cuid") {
        store.send(.testCuid)
      }
      Button("test api client") {
        store.send(.testApiClient)
      }
    }
  }
}

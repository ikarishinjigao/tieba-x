import AppFeature
import SwiftUI

@main
struct TiebaxApp: App {
  @UIApplicationDelegateAdaptor(TiebaxAppDelegate.self) var appDelegate

  var body: some Scene {
    WindowGroup {
      AppView(store: appDelegate.store)
    }
  }
}

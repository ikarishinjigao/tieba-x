import AppFeature
import ComposableArchitecture
import UIKit

final class TiebaxAppDelegate: NSObject {
  lazy var store = Store(
    initialState: AppReducer.State()
  ) {
    AppReducer()
  }
}

extension TiebaxAppDelegate: UIApplicationDelegate {
  func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
  ) -> Bool {
    true
  }
}

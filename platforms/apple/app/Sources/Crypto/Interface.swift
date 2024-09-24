import Dependencies
import DependenciesMacros

@DependencyClient
public struct IDManager {
  public var generateUUID: @Sendable () -> String = { "" }
  public var generateAndroidID: @Sendable (_ seed: UInt64) -> String = { _ in "" }
  public var generateCUID: @Sendable (_ androidID: String) -> String = { _ in "" }
  public var generateC3AID: @Sendable (_ androidID: String, _ uuid: String) -> String = { _, _ in "" }
}

extension DependencyValues {
  public var idManager: IDManager {
    get { self[IDManager.self] }
    set { self[IDManager.self] = newValue }
  }
}

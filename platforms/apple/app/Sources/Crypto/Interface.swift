import Dependencies
import DependenciesMacros

@DependencyClient
public struct CuidManager {
  public var generateCuid: @Sendable () -> String = { "" }
}

extension DependencyValues {
  public var cuidManager: CuidManager {
    get { self[CuidManager.self] }
    set { self[CuidManager.self] = newValue }
  }
}

import Dependencies

extension IDManager: TestDependencyKey {
  public static let previewValue = Self()
  public static let testValue = Self()
}

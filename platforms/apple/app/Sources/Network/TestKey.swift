import Dependencies

extension APIClient: TestDependencyKey {
  public static let previewValue = Self()
  public static let testValue = Self()
}

import Dependencies

extension CuidManager: TestDependencyKey {
  public static let previewValue = Self()
  public static let testValue = Self()
}

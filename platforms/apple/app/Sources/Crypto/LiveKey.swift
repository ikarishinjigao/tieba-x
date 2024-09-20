import Dependencies

extension CuidManager: DependencyKey {
  public static let liveValue = Self(
    generateCuid: { Cuid().generateCuid(androidId: "52ee55117d525049") }
  )
}

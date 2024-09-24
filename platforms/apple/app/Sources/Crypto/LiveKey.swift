import Dependencies

extension IDManager: DependencyKey {
  public static let liveValue = {
    let manager = IdManager()
    return Self(
      generateUUID: { manager.generateUuid() },
      generateAndroidID: { manager.generateAndroidId(seed: $0) },
      generateCUID: { manager.generateCuid(androidId: $0) },
      generateC3AID: { manager.generateC3Aid(androidId: $0, uuid: $1) }
    )
  }()
}

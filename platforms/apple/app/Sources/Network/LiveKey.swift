import Dependencies

final actor Actor {
  let client = ApiClient()
}

extension APIClient: DependencyKey {
  public static let liveValue = {
    let actor = Actor()
    return Self(
      getThreads: { request in
        try await actor.client.getThreads(request: request)
      }
    )
  }()
}

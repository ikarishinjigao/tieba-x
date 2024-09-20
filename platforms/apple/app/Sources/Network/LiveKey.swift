import Dependencies

final actor Actor {
  let apiClient = ApiClient()
}

extension APIClient: DependencyKey {
  public static let liveValue = {
    let actor = Actor()
    return Self(
      getThreads: { request in
        try await actor.apiClient.getThreads(request: request)
      }
    )
  }()
}

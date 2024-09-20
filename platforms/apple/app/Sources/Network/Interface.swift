import Dependencies
import DependenciesMacros

@DependencyClient
public struct APIClient {
  public var getThreads: @Sendable (_ request: GetThreadsRequest) async throws -> GetThreadsResponse
}

extension DependencyValues {
  public var apiClient: APIClient {
    get { self[APIClient.self] }
    set { self[APIClient.self] = newValue }
  }
}

import ComposableArchitecture
import Crypto
import Network

@Reducer
public struct AppReducer {
  @ObservableState
  public struct State: Equatable {
    public init() {}
  }

  public enum Action {
    case testCuid
    case testApiClient
  }

  @Dependency(\.cuidManager) var cuidManager
  @Dependency(\.apiClient) var apiClient

  public init() {}

  public var body: some ReducerOf<Self> {
    Reduce(core)
  }

  func core(into state: inout State, action: Action) -> Effect<Action> {
    switch action {
    case .testCuid:
      .run { send in
        let result = cuidManager.generateCuid()
        print(result)
      }

    case .testApiClient:
      .run { send in
        do {
          let result = try await apiClient.getThreads(
            request: .init(
              forumName: "amd",
              pageNumber: 1,
              pageSize: 5,
              sortType: .createTime,
              isHighQualityThread: false
            )
          )
          print(result)
        } catch {
          print(error)
        }
      }
    }
  }
}

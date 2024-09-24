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

  @Dependency(\.idManager) var idManager
  @Dependency(\.apiClient) var apiClient

  public init() {}

  public var body: some ReducerOf<Self> {
    Reduce(core)
  }

  func core(into state: inout State, action: Action) -> Effect<Action> {
    switch action {
    case .testCuid:
      .run { _ in
        let seed = UInt64.random(in: 0 ... UInt64.max)
        let uuid = idManager.generateUUID()
        let androidID = idManager.generateAndroidID(seed: seed)
        let cuid = idManager.generateCUID(androidID: androidID)
        let c3AID = idManager.generateC3AID(androidID: androidID, uuid: uuid)
        print(androidID)
        print(cuid)
        print(c3AID)
      }

    case .testApiClient:
      .run { _ in
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

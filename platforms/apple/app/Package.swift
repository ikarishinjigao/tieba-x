// swift-tools-version: 5.10

import Foundation
import PackageDescription

let dependencies: [Package.Dependency] = [
  .package(url: "https://github.com/pointfreeco/swift-composable-architecture", from: "1.0.0"),
  .package(url: "https://github.com/pointfreeco/swift-dependencies", from: "1.0.0"),
]

let package = Package(
  name: .packageName,
  platforms: [
    .iOS(.v16),
  ],
  products: [
    .appFeature,
    .crypto,
    .network,
  ],
  dependencies: dependencies,
  targets: [
    .target(
      name: .appFeature,
      dependencies: [
        .byName(name: .crypto),
        .byName(name: .network),
        .composableArchitecture,
      ]
    ),
    .target(
      name: .crypto,
      dependencies: [
        .byName(name: .ffi),
        .dependencies,
        .dependenciesMacros,
      ]
    ),
    .target(
      name: .network,
      dependencies: [
        .byName(name: .ffi),
        .dependencies,
        .dependenciesMacros,
      ]
    ),
    .binaryTarget(name: "FFI", path: "./Frameworks/TiebaxFFI.xcframework"),
  ]
)

extension Product {
  static let appFeature = library(name: .appFeature, targets: [.appFeature])
  static let crypto = library(name: .crypto, targets: [.crypto])
  static let network = library(name: .network, targets: [.network])
}

extension Target.Dependency {
  static let composableArchitecture = product(name: "ComposableArchitecture", package: "swift-composable-architecture")
  static let dependencies = product(name: "Dependencies", package: "swift-dependencies")
  static let dependenciesMacros = product(name: "DependenciesMacros", package: "swift-dependencies")
}

extension String {
  static let packageName = "TiebaX"

  static let ffi = "FFI"

  static let appFeature = "AppFeature"
  static let crypto = "Crypto"
  static let network = "Network"
}

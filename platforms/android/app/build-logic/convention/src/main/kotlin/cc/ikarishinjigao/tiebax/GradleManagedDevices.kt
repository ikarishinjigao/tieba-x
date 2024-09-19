package cc.ikarishinjigao.tiebax

import com.android.build.api.dsl.CommonExtension
import com.android.build.api.dsl.ManagedVirtualDevice
import org.gradle.kotlin.dsl.get
import org.gradle.kotlin.dsl.invoke

internal fun configureGradleManagedDevices(
  commonExtension: CommonExtension<*, *, *, *, *, *>,
) {
  val defaultDevice = DeviceConfig("Pixel 4", 33, "aosp-atd")

  val allDevices = listOf(
    defaultDevice,
  )

  @Suppress("UnstableApiUsage")
  commonExtension.testOptions {
    managedDevices {
      devices {
        allDevices.forEach {
          maybeCreate(it.taskName, ManagedVirtualDevice::class.java).apply {
            device = it.device
            apiLevel = it.apiLevel
            systemImageSource = it.systemImageSource
          }
        }
        groups {
          maybeCreate("default").apply {
            allDevices.forEach {
              targetDevices.add(devices[it.taskName])
            }
          }
        }
      }
    }
  }
}

private data class DeviceConfig(
  val device: String,
  val apiLevel: Int,
  val systemImageSource: String,
) {
  val taskName = buildString {
    append(device.lowercase().replace(" ", ""))
    append("api")
    append(apiLevel.toString())
    append(systemImageSource.replace("-", ""))
  }
}

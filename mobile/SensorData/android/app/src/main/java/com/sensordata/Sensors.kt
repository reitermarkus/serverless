package com.sensorData

import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager

class Sensors(private val manager: SensorManager) : SensorEventListener {
  companion object {
    private var _sensorData: HashMap<String, String> = HashMap()
  }

  init {
    for (sensor in getSensors()) {
      manager.registerListener(this, sensor, SensorManager.SENSOR_DELAY_NORMAL)
    }
  }

  override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) { }
  override fun onSensorChanged(event: SensorEvent) = parseSensorInfo(event)

  public val sensorInfo: HashMap<String, String>
    get() = _sensorData

  public fun getSensors() : List<Sensor> = manager.getSensorList(Sensor.TYPE_ALL)

  public fun parseSensorInfo(event: SensorEvent) {
    val type = event.sensor.type

    when (type) {
      Sensor.TYPE_ACCELEROMETER, Sensor.TYPE_LINEAR_ACCELERATION ->
        _sensorData.set(
          "Acceleration",
          "x=${event.values[0]}m/s² y=${event.values[1]}m/s² z=${event.values[2]}m/s²"
        )
      Sensor.TYPE_GRAVITY ->
        _sensorData.set(
          "Gravity",
          "x=${event.values[0]}m/s² y=${event.values[1]}m/s² z=${event.values[2]}m/s²"
        )
      Sensor.TYPE_GYROSCOPE ->
        _sensorData.set(
          "Gyroscope",
          "x=${event.values[0]}rad/s Y=${ event.values[1]}rad/s z=${event.values[2]}rad/s"
        )
      Sensor.TYPE_ROTATION_VECTOR ->
        _sensorData.set(
          "Rotation",
          "x=${event.values[0]} y=${event.values[1]} z=${event.values[2]}"
        )
      Sensor.TYPE_MAGNETIC_FIELD ->
        _sensorData.set(
          "Rotation",
          "x=${event.values[0]}μT y=${event.values[1]}μT z=${event.values[2]}μT"
        )
      Sensor.TYPE_ORIENTATION ->
        _sensorData.set(
          "Rotation",
          "Azimuth=${event.values[0]}° Pitch=${event.values[1]}° Roll=${event.values[2]}°"
        )
      Sensor.TYPE_PROXIMITY ->
        _sensorData.set(
          "Proximity",
          "${event.values[0]}cm"
        )
      Sensor.TYPE_AMBIENT_TEMPERATURE ->
        _sensorData.set(
          "Air temperature",
          "${event.values[0]}°C"
        )
      Sensor.TYPE_LIGHT ->
        _sensorData.set(
          "Illuminance",
          "${event.values[0]}lx"
        )
      Sensor.TYPE_PRESSURE ->
        _sensorData.set(
          "Air pressure",
          "${event.values[0]}hPa"
        )
      Sensor.TYPE_RELATIVE_HUMIDITY ->
        _sensorData.set(
          "Relative humidity",
          "${event.values[0]}%"
        )
      Sensor.TYPE_GYROSCOPE_UNCALIBRATED ->
        _sensorData.set(
          "Gyroscope uncalibrated",
          "x=${event.values[0]}rad/s y=${event.values[1]}rad/s z=${event.values[2]}rad/s"
        )
      Sensor.TYPE_MAGNETIC_FIELD_UNCALIBRATED ->
        _sensorData.set(
          "Magnetics uncalibrated",
          "x=${event.values[0]}μT y=${event.values[1]}μT z=${event.values[2]}μT"
        )
      Sensor.TYPE_GEOMAGNETIC_ROTATION_VECTOR ->
        _sensorData.set(
          "Geomagnetic rotation",
          "x=${event.values[0]} y=${event.values[1]} z=${event.values[2]}"
        )
    }
  }
}
package com.example.sensor_data

import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager

import org.json.JSONObject

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

  public fun asJson(): JSONObject {
    val sensorsJson = JSONObject()

    _sensorData.forEach { (key, value) ->
      val splitted = value.split(" ")

      if (splitted.size > 1) {
        val obj = JSONObject()

        for (sensorVal in splitted) {
          val pair = sensorVal.split("=")

          if (pair.size == 2) {
            obj.put(pair[0], pair[1])
          }
        }

        sensorsJson.put(key, obj)
      } else {
        sensorsJson.put(key, value)
      }
    }

    return sensorsJson
  }

  public fun parseSensorInfo(event: SensorEvent) {
    val type = event.sensor.type

    when (type) {
      Sensor.TYPE_ACCELEROMETER, Sensor.TYPE_LINEAR_ACCELERATION ->
        _sensorData.set(
          "acceleration",
          "x=${event.values[0]}m/s² y=${event.values[1]}m/s² z=${event.values[2]}m/s²"
        )
      Sensor.TYPE_GRAVITY ->
        _sensorData.set(
          "gravity",
          "x=${event.values[0]}m/s² y=${event.values[1]}m/s² z=${event.values[2]}m/s²"
        )
      Sensor.TYPE_GYROSCOPE ->
        _sensorData.set(
          "gyroscope",
          "x=${event.values[0]}rad/s y=${ event.values[1]}rad/s z=${event.values[2]}rad/s"
        )
      Sensor.TYPE_ROTATION_VECTOR ->
        _sensorData.set(
          "rotation",
          "x=${event.values[0]} y=${event.values[1]} z=${event.values[2]}"
        )
      Sensor.TYPE_MAGNETIC_FIELD ->
        _sensorData.set(
          "magnetic",
          "x=${event.values[0]}μT y=${event.values[1]}μT z=${event.values[2]}μT"
        )
      Sensor.TYPE_ORIENTATION ->
        _sensorData.set(
          "orientation",
          "azimuth=${event.values[0]}° pitch=${event.values[1]}° roll=${event.values[2]}°"
        )
      Sensor.TYPE_PROXIMITY ->
        _sensorData.set(
          "proximity",
          "${event.values[0]}cm"
        )
      Sensor.TYPE_AMBIENT_TEMPERATURE ->
        _sensorData.set(
          "air_temperature",
          "${event.values[0]}°C"
        )
      Sensor.TYPE_LIGHT ->
        _sensorData.set(
          "illuminance",
          "${event.values[0]}lx"
        )
      Sensor.TYPE_PRESSURE ->
        _sensorData.set(
          "air_pressure",
          "${event.values[0]}hPa"
        )
      Sensor.TYPE_RELATIVE_HUMIDITY ->
        _sensorData.set(
          "relative_humidity",
          "${event.values[0]}%"
        )
      Sensor.TYPE_GYROSCOPE_UNCALIBRATED ->
        _sensorData.set(
          "gyroscope_uncalibrated",
          "x=${event.values[0]}rad/s y=${event.values[1]}rad/s z=${event.values[2]}rad/s"
        )
      Sensor.TYPE_MAGNETIC_FIELD_UNCALIBRATED ->
        _sensorData.set(
          "magnetics_uncalibrated",
          "x=${event.values[0]}μT y=${event.values[1]}μT z=${event.values[2]}μT"
        )
      Sensor.TYPE_GEOMAGNETIC_ROTATION_VECTOR ->
        _sensorData.set(
          "geomagnetic_rotation",
          "x=${event.values[0]} y=${event.values[1]} z=${event.values[2]}"
        )
    }
  }
}
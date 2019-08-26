import { DEVICES, DEVICE_DATA } from './samples'
import axios from 'axios'
import UIkit from 'uikit'

export default class {
  onCreate() {
    this.state = {
      devices: [],
      deviceData: {},
    }

    this.connected = null
    this.notification = null

    this.fetchData()
  }

  async fetchData() {
    try {
      const {data: devices} = await axios.get('/function/devices')

      console.log(devices)

      this.state = {
        devices: [...DEVICES, ...devices],
        deviceData: DEVICE_DATA,
        currentDevice: 0,
      }

      if (this.connected === false) {
        this.notification?.close()
        this.notification = UIkit.notification('Connected.', {status: 'success'})
      }

      this.connected = true
      setTimeout(() => this.fetchData(), 60000)
    } catch(_) {
      this.connected = false
      this.notification?.close()

      this.notification = UIkit.notification('Connecting …', {status: 'primary'})

      setTimeout(() => this.fetchData(), 15000)
    }
  }

  async updateInterval(start, end) {
    const device = this.state.devices[this.state.currentDevice]

    const data = await Promise.all(device.data_types.map(async dataType => {
      const { data } = await axios.post('/function/filter', {
        'device_id': device.id,
        'collection': dataType,
        'begin': start.toISOString(),
        'end': end.toISOString(),
      })

      return {
        label: dataType,
        data: data.map(({ value, time }) => ({ x: new Date(time), y: value })),
      }
    }))

    this.state.deviceData[device.id] = {
      datasets: data.map(data => ({
        ...data,
        fill: false,
      })),
      type: 'line',
      options: { scales: { xAxes: [{ type: 'time' }] } },
    }

    this.setStateDirty('deviceData')
  }

  async handleDeviceChange(id) {
    this.state.currentDevice = id

    const device = this.state.devices[id]

    const data = await Promise.all(device.data_types.map(async dataType => {
      const { data } = await axios.post('/function/filter', {
        'device_id': device.id,
        'collection': dataType,
      })

      return {
        label: dataType,
        data: data.map(({ value, time }) => ({ x: new Date(time), y: value })),
      }
    }))

    this.state.deviceData[device.id] = {
      datasets: data.map(data => ({
        ...data,
        fill: false,
      })),
      type: 'line',
      options: {scales: {xAxes: [{type: 'time'}]}},
    }

    this.setStateDirty('deviceData')
  }
}

import { DEVICES, DEVICE_DATA } from './samples'
import axios from 'axios'
import UIkit from 'uikit'

export default class {
  onCreate() {
    this.state = {
      devices: [],
      deviceData: {},
      stepSlider: 24
    }

    this.connected = null
    this.notification = null

    this.fetchData()
  }

  updateStepSlider(e) {
    this.state.stepSlider = e.srcElement?.valueAsNumber
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

    const datasetsArray = await Promise.all(device.data_types.map(async dataType => {
      const { data } = await axios.post('/function/filter', {
        'device_id': device.id,
        'collection': dataType,
        'begin': start.toISOString(),
        'end': end.toISOString(),
        'interval': this.state.stepSlider || 24
      })

      if (data.every(d => d.avg != null)) {
        return [{
          label: dataType,
          data: data.map(({ avg, time }) => ({ x: new Date(time), y: avg })),
        }]
      } else {
        return data.reduce((acc, {avg_x, avg_y, avg_z, time}) => {
          acc[0].data.push({ x: new Date(time), y: avg_x })
          acc[1].data.push({ x: new Date(time), y: avg_y })
          acc[2].data.push({ x: new Date(time), y: avg_z })
          return acc
        }, [{label: 'x', data: []}, {label: 'y', data: []}, {label: 'z', data: []}])
      }
    }))

    const charts = datasetsArray.filter(datasets => datasets.some(d => d.data.length > 0)).map(datasets => {
      return {
        chartType: 'line',
        chart: {
          datasets: datasets.map(dataset => ({...dataset, fill: false})),
          type: 'line',
          options: { scales: { xAxes: [{ type: 'time' }] } },
        }
      }
    })

    this.state.deviceData[device.id] = charts
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

    const charts = data.filter(d => d.data.length > 0).map(d => {
      return {
        chartType: 'line',
        chart: {
          datasets: [{
            ...d,
            fill: false
          }],
          type: 'line',
          options: { scales: { xAxes: [{ type: 'time' }] } },
        }
      }
    })

    this.state.deviceData[device.id] = charts
    this.setStateDirty('deviceData')
  }
}

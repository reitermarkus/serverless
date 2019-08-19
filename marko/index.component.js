import { DEVICES } from './samples'
import axios from 'axios'
import UIkit from 'uikit'

export default class {
  onCreate() {
    this.state = {
      devices: [],
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

  async handleDeviceChange(id) {
    this.state.currentDevice = id

    const device = this.state.devices[id]

    const data = await Promise.all(device.data_types.map(async d => {
      const { data } = await axios.post('/function/filter', {
        'device_id': device.id,
        'collection': d,
      })

      return data
    }))

    console.log(data)
  }
}

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

  handleDeviceChange(id) {
    this.state.currentDevice = id
  }
}

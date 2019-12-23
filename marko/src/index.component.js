import axios from 'axios'
import UIkit from 'uikit'
import Pikaday from 'pikaday'

export default class {
  onCreate() {
    this.state = {
      devices: [],
      deviceData: {},
      currentDevice: null,
      stepSlider: 24,
      loading: false,
      pickerStart: null,
      pickerEnd: null,
      headless: false
    }

    this.connected = null
    this.notification = null

    this.fetchData()

    this.pickerStart = null
    this.pickerEnd = null

    const urlParams = new URLSearchParams(window.location.search)

    if (urlParams.has('headless') && Boolean(urlParams.get('headless'))) {
      this.state.headless = true
    }
  }

  onUpdate() {
    if (!this.pickerStart) {
      const dp = document.getElementById('datepickerStart')

      if(dp)
        this.pickerStart = new Pikaday({
          field: dp,
          maxDate: new Date(),
          defaultDate: new Date(new Date(Date.now()).getTime() - (1 * 24 * 60 * 60 * 1000)),
          setDefaultDate: true,
          onSelect: date => this.state.pickerStart = date
        })
    }

    if (!this.pickerEnd) {
      const dp = document.getElementById('datepickerEnd')

      if (dp)
        this.pickerEnd = new Pikaday({
          field: dp,
          maxDate: new Date(),
          defaultDate: new Date(Date.now()),
          setDefaultDate: true,
          onSelect: date => {
            const now = new Date()
            now.setHours(0, 0, 0, 0)

            if (date.getTime() === now.getTime()) {
              this.state.pickerEnd = new Date()
            } else {
              this.state.pickerEnd = date
            }
          }
        })
    }
  }

  updateStepSlider(e) {
    this.state.stepSlider = e.srcElement?.valueAsNumber
  }

  getDates() {
    const end = this.pickerEnd?._d || new Date(Date.now())
    const start = this.pickerStart?._d || new Date(end.getTime() - (1 * 24 * 60 * 60 * 1000))

    return {
      start: start,
      end: end
    }
  }

  async fetchData() {
    try {
      let {data: devices} = await axios.get('/function/devices')
      devices = devices.reduce((acc, d) => { acc[d.id] = d; return acc }, {})
      this.state.devices = devices

      if (devices !== [] && this.state.currentDevice == null) {
        const currentDeviceId = location.hash.substring(1) || Object.keys(devices)[0]
        this.state.currentDevice = currentDeviceId
        this.handleDeviceChange(this.state.currentDevice)
      } else {
        const { start, end } = this.getDates()
        this.updateInterval(start, end, true)
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

  async updateInterval(start, end, silent) {
    const device = this.state.devices[this.state.currentDevice]

    if (!silent) {
      this.state.loading = true
    }

    const datasetsArray = await Promise.all(device.data_types.map(async dataType => {
      const { data } = await axios.post('/function/filter', {
        'device_id': device.id,
        'collection': dataType,
        'begin': start.toISOString(),
        'end': end.toISOString(),
        'interval': this.state.stepSlider || 24
      })

      if (data.every(d => d.avg != null)) {
        return {
          title: dataType,
          chart: [{
            label: dataType,
            data: data.map(({ avg, time }) => ({ x: new Date(time), y: avg })),
          }]
        }
      } else {
        return {
          title: dataType,
          chart: data.reduce((acc, {avg_x, avg_y, avg_z, time}) => {
            acc[0].data.push({ x: new Date(time), y: avg_x })
            acc[1].data.push({ x: new Date(time), y: avg_y })
            acc[2].data.push({ x: new Date(time), y: avg_z })
            return acc
          }, [{label: 'x', data: []}, {label: 'y', data: []}, {label: 'z', data: []}])
        }
      }
    }))

    const charts = datasetsArray.map(datasets => {
      return {
        title: datasets.title,
        chartContainer: {
          chartType: 'line',
          chart: {
            datasets: datasets.chart.map(dataset => ({...dataset, fill: false})),
            type: 'line',
            options: {
              scales: {
                xAxes: [
                  { type: 'time' }
                ]
              }
            },
          }
        }
      }
    })

    if (!silent) {
      this.state.loading = false
    }

    this.state.deviceData[device.id] = charts
    this.setStateDirty('deviceData')
  }

  async handleDeviceChange(id) {
    this.state.currentDevice = id

    const {start, end} = this.getDates()

    if (this.pickerStart) {
      this.pickerStart.destroy()
      this.pickerStart = null
    }

    if (this.pickerEnd) {
      this.pickerEnd.destroy()
      this.pickerEnd = null
    }

    this.updateInterval(start, end, false)
  }
}

import axios from 'axios'
import UIkit from 'uikit'

export default class {
  onCreate() {
    const startDate = new Date(new Date(Date.now()).getTime() - (1 * 24 * 60 * 60 * 1000))
    const endDate = new Date()

    this.state = {
      devices: [],
      deviceData: {},
      currentDevice: null,
      stepSlider: 24,
      loading: false,
      headless: false,
      startDate: startDate,
      endDate: endDate,
      currentStartDate: startDate,
      currentEndDate: endDate,
    }

    this.connected = null
    this.notification = null

    const urlParams = new URLSearchParams(window.location.search)

    if (urlParams.has('headless') && Boolean(urlParams.get('headless'))) {
      this.state.headless = true
    }

    this.fetchData()
  }

  handleStartDateChange(date) {
    this.state.currentStartDate = date
  }

  handleEndDateChange(date) {
    this.state.currentEndDate = date
  }

  updateStepSlider(e) {
    this.state.stepSlider = e.srcElement?.valueAsNumber
  }

  getDates() {
    return {
      start: this.state.currentStartDate,
      end: this.state.currentEndDate
    }
  }

  async fetchData() {
    try {
      let {data: devices} = await axios.get('/function/devices')
      devices = devices.reduce((acc, d) => { acc[d.id] = d; return acc }, {})
      this.state.devices = devices

      if (devices !== {} && this.state.currentDevice == null) {
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
    this.state.currentStartDate = start
    this.state.currentEndDate = end

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

    this.updateInterval(start, end, false)
  }
}

import noUiSlider from 'nouislider'

export default class {
  onCreate() {
    this.state = {
      start: null,
      end: null,
    }
  }

  updateInterval() {
    this.emit('update-interval', this.state.start, this.state.end)
  }

  onMount() {
    const dateSlider = document.querySelector('.slider-date')

    const now = new Date(Date.now())
    const yesterday = new Date(now.getTime() - (1 * 24 * 60 * 60 * 1000)).getTime()
    const previousWeek = new Date(now.getTime() - (7 * 24 * 60 * 60 * 1000)).getTime()

    noUiSlider.create(dateSlider, {
      range: {
        min: previousWeek,
        max: now.getTime()
      },
      connect: true,

      // Steps of one day
      step: 60 * 60 * 1000,
      start: [yesterday, now.getTime()],
    })

    const updateState = (idx, date) => {
      if (idx === 0) {
        this.state.start = date
      } else {
        this.state.end = date
      }
    }

    dateSlider.noUiSlider.on('update', function (values, handle) {
      updateState(handle, new Date(+values[handle]))
    })
  }
}

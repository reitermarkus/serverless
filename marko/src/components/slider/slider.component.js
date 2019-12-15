import noUiSlider from 'nouislider'

export default class {
  onCreate() {
    this.state = {
      start: null,
      end: null,
    }

    this.dateUiSlider = null
  }

  updateInterval() {
    this.emit('update-interval', this.state.start, this.state.end)
  }

  onInput(input) {
    this.state.start = input.start || this.state.start
    this.state.end = input.end || this.state.end

    if(this.state.start && this.state.end) {
      this.dateUiSlider && this.dateUiSlider.updateOptions({
        start: [
          this.state.start.getTime(),
          this.state.end.getTime()
        ],
        range: {
          min: this.state.start.getTime(),
          max: this.state.end.getTime()
        },
      })
    }
  }

  onMount() {
    const dateSlider = document.querySelector('.slider-date')

    const now = new Date(Date.now())
    const yesterday = new Date(now.getTime() - (1 * 24 * 60 * 60 * 1000)).getTime()

    this.dateUiSlider = noUiSlider.create(dateSlider, {
      range: {
        min: yesterday,
        max: now.getTime()
      },
      connect: true,

      // Steps of one day
      step: 60 * 60 * 1000,
      start: [yesterday, now.getTime()]
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

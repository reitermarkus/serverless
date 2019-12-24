import noUiSlider from 'nouislider'

export default class {
  onCreate() {
    this.state = {
      start: null,
      currentStart: null,
      end: null,
      currentEnd: null,
    }

    this.dateUiSlider = null
  }

  updateInterval() {
    this.emit('update-interval', this.state.currentStart, this.state.currentEnd)
  }

  onInput(input) {
    this.state.start = input.start
    this.state.currentStart = input.currentStart
    this.state.end = input.end
    this.state.currentEnd = input.currentEnd
  }

  onUpdate() {
    this.unbindHandler()

    this.dateSlider.noUiSlider.updateOptions({
      start: [
        this.state.currentStart.getTime(),
        this.state.currentEnd.getTime()
      ],
      range: {
        min: this.state.start.getTime(),
        max: this.state.end.getTime()
      },
    })

    this.bindHandler()
  }

  unbindHandler() {
    this.dateSlider.noUiSlider.off('set')
  }

  bindHandler() {
    this.dateSlider.noUiSlider.on('set', (values, handle) => {
      const date = new Date(+values[handle])

      if (handle === 0) {
        if (this.state.currentStart !== date) {
          this.state.currentStart = date
        }
      } else {
        if (this.state.currentEnd !== date) {
          this.state.currentEnd = date
        }
      }
    })
  }

  onMount() {
    this.dateSlider = this.getEl('slider')

    const now = new Date(Date.now())
    const yesterday = new Date(now.getTime() - (1 * 24 * 60 * 60 * 1000)).getTime()

    noUiSlider.create(this.dateSlider, {
      range: {
        min: yesterday,
        max: now.getTime()
      },
      connect: true,

      // Steps of one day
      step: 60 * 60 * 1000,
      start: [yesterday, now.getTime()]
    })

    this.bindHandler()
  }
}

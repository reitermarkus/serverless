import Pikaday from 'pikaday'

export default class {
  onCreate() {
    this.state = {
      start: null,
      end: null,
    }
  }

  onMount() {
    this.picker = new Pikaday({
      field: this.getEl('picker'),
      maxDate: this.state.end,
      defaultDate: this.state.start,
      setDefaultDate: true,
      onSelect: date => {
        this.emit('date-change', date)
      },
    })
  }

  onUpdate() {
    this.picker.defaultDate = this.state.start
    this.picker.maxDate = this.state.end
  }

  onInput(input) {
    this.state = {
      start: input.start,
      end: input.end,
    }
  }
}

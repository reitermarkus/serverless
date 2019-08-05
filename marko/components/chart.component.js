import Chart from 'chart.js'

export default class {
  onInput(input) {
    const title = input.title ? { title: { display: true, text: input.title } } : {}

    var options = input.options || {}

    // Start bar charts at 0.
    if (input.type == 'bar') {
      options.scales = {
        yAxes: [{
          ticks: {
            beginAtZero: true
          }
        }]
      }
    }

    this.state = {
      type: input.type,
      data: input.data,
      options: {
        ...title,
        ...options,
      }
    }
  }

  onMount() {
    const canvas = this.getEl("canvas")
    const ctx = canvas.getContext('2d')

    this.chart = new Chart(ctx, {
      type: this.state.type,
      data: this.state.data,
      options: this.state.options,
    })
  }

  onUpdate() {
    this.chart.type = this.state.type
    this.chart.data = this.state.data
    this.chart.options = this.state.options

    this.chart.update()
  }
}

import Chart from 'chart.js'

export default class {
  onInput(input) {
    const title = input.title ? { title: { display: true, text: input.title } } : {}

    let options = input.options || {}

    // Start bar charts at 0.
    if (input.type == 'bar') {
      options = Chart.helpers.merge(options, {
        scales: {
          yAxes: [{
            ticks: {
              beginAtZero: true
            }
          }]
        },
      })
    }

    // Don't show legend if no labels are set.
    if (input.data.datasets.every((dataset) => dataset.label == null)) {
      options = Chart.helpers.merge(options, {
        legend: {
          display: false,
        },
      })
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
    const canvas = this.getEl('canvas')
    const ctx = canvas.getContext('2d')

    this.chart = new Chart(ctx, {
      type: this.state.type,
      data: this.state.data,
      options: this.state.options,
    })
  }

  onUpdate() {
    this.chart.config.type = this.state.type

    this.chart.options = this.state.options
    this.chart.data.labels = this.state.data.labels

    let indexedByLabel = this.chart.data.datasets.reduce((map, d) => {
      map[d.label] = d
      return map
    }, {})

    this.chart.data.datasets = this.state.data.datasets.map(next => {
      const { _meta } = indexedByLabel[next.label]
      return {_meta: _meta, ...next}
    })

    this.chart.update()
  }
}

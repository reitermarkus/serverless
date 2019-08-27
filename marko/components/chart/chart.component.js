import Chart from 'chart.js'
import Color from 'color'
import colors from '../../style.scss'

const BASE_COLORS = Object.values(colors).map(c => Color(c))
const BACKGROUND_COLORS = BASE_COLORS.map(c => c.alpha(0.3).string())
const BACKGROUND_BORDER_COLORS = BASE_COLORS.map(c => c.string())

export default class {
  onInput(input) {
    let datasets = input.data.datasets

    if (datasets.length === 1 && input.type !== 'line') {
      datasets.forEach(d => {
        d.backgroundColor = d.data.map((_, i) => BACKGROUND_COLORS[i % BACKGROUND_COLORS.length])
        d.borderColor = d.data.map((_, i) => BACKGROUND_BORDER_COLORS[i % BACKGROUND_BORDER_COLORS.length])
        d.borderWidth = 1
      })
    } else {
      datasets.forEach((d, i) => {
        d.backgroundColor = BACKGROUND_COLORS[i % BACKGROUND_COLORS.length]
        d.borderColor = BACKGROUND_BORDER_COLORS[i % BACKGROUND_BORDER_COLORS.length]
        d.borderWidth = 1
      })
    }

    let options = input.options || {}

    options.onClick = input.onClick
    options.onHover = input.onHover
    options.maintainAspectRatio = false
    options.aspectRatio = 1.0

    // Start bar charts at 0.
    if (input.type === 'bar') {
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
      options: options,
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
      const { _meta } = indexedByLabel[next.label] || {}
      return {_meta: _meta, ...next}
    })

    this.chart.update()
  }
}

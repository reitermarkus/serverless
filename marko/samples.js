export const DEVICES = [
  {
    name: 'Raspberry Pi 1',
    id: 'rpi1',
  },
  {
    name: 'Raspberry Pi 2',
    id: 'rpi2',
  },
  {
    name: 'Raspberry Pi 3',
    id: 'rpi3',
  },
  {
    name: 'Android Device 1',
    id: 'android1',
  },
  {
    name: 'iOS Device 1',
    id: 'ios1',
  },
]

export const DEVICE_DATA = {
  'rpi1': {
    datasets: [{
      label: 'Pressure',
      fill: false,
      data: [
        {
          x: new Date('2019-07-28T18:23:33Z'),
          y: 8,
        },
        {
          x: new Date('2019-07-28T18:24:03Z'),
          y: 2,
        },
        {
          x: new Date('2019-07-28T18:24:33Z'),
          y: 2,
        },
        {
          x: new Date('2019-07-28T18:25:03Z'),
          y: 44,
        },
        {
          x: new Date('2019-07-28T18:25:33Z'),
          y: 33,
        },
        {
          x: new Date('2019-07-28T18:26:03Z'),
          y: 26,
        },
      ]
    }],
    type: 'line',
    options: {scales: {xAxes: [{type: 'time'}]}},
  },
  'rpi2': {
    labels: ['Red', 'Blue', 'Yellow', 'Green'],
    datasets: [{data: [32, 11,  7,  7]}],
    type: 'bar',
  },
  'rpi3': {
    labels: ['Red', 'Blue', 'Yellow'],
    datasets: [{data: [3, 5, 2]}],
    type: 'doughnut',
  },
  'android1': {
    labels: ['Red', 'Blue'],
    datasets: [{data: [47, 43]}],
    type: 'bar',
  },
  'ios1': {
    labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple'],
    datasets: [{data: [19, 28, 18, 21, 20]}],
    type: 'pie',
  }
}

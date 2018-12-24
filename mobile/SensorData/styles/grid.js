import { StyleSheet, Dimensions } from 'react-native'

const { width, _ } = Dimensions.get('window')

export const grid = StyleSheet.create({
  container: {
    marginBottom: 15
  },
  item: {
    marginLeft: 'auto',
    marginRight: 'auto',
    width: width * 0.7 / 2,
    justifyContent: 'center',
    paddingBottom: 5
  },
  list: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    flexWrap: 'wrap'
  },
})

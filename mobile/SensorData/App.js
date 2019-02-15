/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { Platform } from 'react-native'
import AndroidView from './androidView'
import IosView from './iosView'

export default class App extends Component {
  constructor(props) {
    super(props)
  }

  render() {
    return (
      <>
        {Platform.OS === 'android' ?
          <AndroidView /> :
          <IosView />
        }
      </>
    )
  }
}

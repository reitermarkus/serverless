/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { Platform } from 'react-native'
import { StyleProvider, Container, Header, Content, Text, Title, Body, Icon, Footer, FooterTab, Button } from 'native-base'

import getTheme from './native-base-theme/components'
import platform from './native-base-theme/variables/platform'

import AndroidView from './androidView'
import IosView from './iosView'
import SettingsPage from './settingsPage'

export default class App extends Component {
  constructor(props) {
    super(props)

    this.state = {
      footerTab: 0
    }
  }

  render() {
    const sensorPage = () =>
      <>
        {Platform.OS === 'android' ?
          <AndroidView /> :
          <IosView />
        }
      </>

    const renderTab = (number) => {
      switch (number) {
        case 0:
          return sensorPage()
        case 1:
          return <SettingsPage />
        default:
          return sensorPage()
      }
    }

    const changeTab = (number) => {
      if (this.state.footerTab !== number) {
        this.setState({ footerTab: number });
      }
    }

    return (
      <StyleProvider style={getTheme(platform)}>
        <Container>
          <Header>
            <Body>
              <Title>Sensor Data</Title>
            </Body>
          </Header>
          <Content>
            {renderTab(this.state.footerTab)}
          </Content>
          <Footer>
            <FooterTab>
              <Button vertical active={this.state.footerTab === 0} onPress={() => changeTab(0) }>
                <Icon type="FontAwesome" name="microchip" />
                <Text>Sensors</Text>
              </Button>
              <Button vertical active={this.state.footerTab === 1} onPress={() => changeTab(1) }>
                <Icon name="settings" />
                <Text>Settings</Text>
              </Button>
            </FooterTab>
          </Footer>
        </Container>
      </StyleProvider>
    )
  }
}

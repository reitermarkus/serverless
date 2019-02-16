/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { Platform } from 'react-native'
import { StyleProvider, Container, Header, Content, Text, Title, Body, Icon, Footer, FooterTab, Button } from 'native-base'

import getTheme from './native-base-theme/components'
import theme from './native-base-theme/variables/platform'

const SensorPage = Platform.OS == 'ios' ? require('./iosView').default : require('./androidView').default
import SettingsPage from './settingsPage'

export default class App extends Component {
  constructor(props) {
    super(props)

    this.state = {
      footerTab: 0
    }
  }

  render() {
    const renderTab = (number) => {
      switch (number) {
        case 0:
          return <SensorPage/>
        case 1:
          return <SettingsPage/>
        default:
          return <SensorPage/>
      }
    }

    const changeTab = (number) => {
      if (this.state.footerTab !== number) {
        this.setState({ footerTab: number });
      }
    }

    return (
      <StyleProvider style={getTheme(theme)}>
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

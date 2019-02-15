import React, {Component} from 'react'
import { Form, Item, Input, Icon, View } from 'native-base'

export default class SettingsPage extends Component {
  constructor(props) {
    super(props)

    this.state = {
      ip: null
    }
  }

  render() {
    return (
      <View>
        <Form>
          <Item>
            <Icon type='FontAwesome' name='server'/>
            <Input
              placeholder="Kafka REST URL"
              value={this.state.ip}
              onChangeText={(text) => this.setState({ip: text})}
              keyboardType={Platform.OS == 'ios' ? 'url' : 'default'}
            />
          </Item>
        </Form>
      </View>
    )
  }
}

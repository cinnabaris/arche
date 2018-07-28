import React, {
  Component
} from 'react'
import {
  injectIntl,
  intlShape,
  FormattedMessage
} from 'react-intl'
import {
  Form,
  Input,
  Menu,
  message
} from 'antd'

import {
  Submit,
  formItemLayout
} from '../../../components/form'
import Card from '../../../components/layouts/Card'
import {
  client,
  failed
} from '../../../utils/request'
import Authorized from '../../../components/Authorized'
import {
  is_administrator
} from '../../../utils/authorized'

const FormItem = Form.Item

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      languages: []
    }
  }
  handleMenuClick = (e) => {
    var win = window.open(e.key, '_blank')
    win.focus()
  }
  handleSubmit = (e) => {
    const {
      formatMessage
    } = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client().request(`mutation form($googleSiteVerifyCode: String!, $baiduSiteVerifyCode: String!){
            updateSiteSeo(googleSiteVerifyCode: $googleSiteVerifyCode, baiduSiteVerifyCode: $baiduSiteVerifyCode) {
              createdAt
            }
          }`, values).then((rst) => {
          message.info(formatMessage({
            id: "flashes.success"
          }))
        }).catch(failed)
      }
    })
  }
  componentDidMount() {
    const {
      setFieldsValue
    } = this.props.form
    client().request(`query info{
        getSiteSeo{
          googleSiteVerifyCode, baiduSiteVerifyCode
        }
        getSiteInfo{
          languages
        }
      }`).then((rst) => {
      setFieldsValue(rst.getSiteSeo)
      this.setState({
        languages: rst.getSiteInfo.languages
      })
    }).catch(() => {})
  }
  render() {
    const {
      formatMessage
    } = this.props.intl
    const {
      getFieldDecorator,
      getFieldValue
    } = this.props.form
    return (<Authorized check={is_administrator}>
      <Card title={{
        id: "nut.admin.site.seo.title"
      }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.seo.google.site-verify-code" />} hasFeedback>
            {
              getFieldDecorator('googleSiteVerifyCode', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id = "nut.attributes.seo.baidu.site-verify-code" />} hasFeedback>
            {
              getFieldDecorator('baiduSiteVerifyCode', {
                rules: [{
                  required: true,
                  message: formatMessage({id: "validations.required"})
                }]
              })(<Input/>)
            }
          </FormItem>
          <Submit/>
        </Form>
        <Menu mode="inline" onClick={this.handleMenuClick}>
          {this.state.languages.map((it)=>`/rss/${it}`).concat([
            '/robots.txt',
            '/sitemap.xml.gz',
            `/google${getFieldValue('googleSiteVerifyCode')}.html`,
            `/baidu_verify_${getFieldValue('baiduSiteVerifyCode')}.html`
          ]).map((it)=>(<Menu.Item key={it}>
            {it}
          </Menu.Item>))}
        </Menu>
      </Card>
    </Authorized>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired
}

export default Form.create()(injectIntl(Widget))
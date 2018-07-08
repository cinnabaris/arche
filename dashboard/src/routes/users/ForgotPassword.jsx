import React from 'react'

import EmailForm from './EmailForm'
import {USERS_FORGOT_PASSWORD} from '../../request'

const Widget = () => (<EmailForm action="forgot-password" query={USERS_FORGOT_PASSWORD}/>)

export default Widget

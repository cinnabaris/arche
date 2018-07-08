import React from 'react'

import EmailForm from './EmailForm'
import {USERS_CONFIRM} from '../../request'

const Widget = () => (<EmailForm action="confirm" query={USERS_CONFIRM}/>)

export default Widget
